use clap::Parser;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

#[derive(Parser)]
#[command(about = "Compare class files between versions")]
struct Args {
    /// Artifact (Maven: org.example/my-lib, GitHub: owner/repo)
    artifact: String,

    /// Start version (optional)
    from: Option<String>,

    /// End version (optional)
    to: Option<String>,

    /// Verbose
    #[arg(short, long)]
    verbose: bool,

    /// Show all items without truncation
    #[arg(short, long)]
    full: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Parse artifact format
    let (group_id, artifact_id, is_github) = parse_artifact(&args.artifact);

    if is_github {
        return analyze_github(
            &group_id,
            &artifact_id,
            args.from.as_deref(),
            args.to.as_deref(),
            args.verbose,
            args.full,
        );
    }

    // Maven mode
    let agent = Arc::new(
        ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(30))
            .build(),
    );

    let repo_url = "https://repo1.maven.org/maven2";
    let local_repo = local_m2_repo()?;

    // Fetch versions
    let versions = fetch_versions(&agent, repo_url, &group_id, &artifact_id)?;
    if versions.is_empty() {
        println!("No versions found");
        return Ok(());
    }

    // Filter by from/to
    let filtered = filter_versions(&versions, args.from.as_deref(), args.to.as_deref());
    if filtered.len() < 2 {
        println!("Need at least 2 versions");
        if !versions.is_empty() {
            println!(
                "Available versions: {} to {}",
                versions[0],
                versions[versions.len() - 1]
            );
            if args.verbose {
                println!("All versions: {}", versions.join(", "));
            }
        }
        return Ok(());
    }

    if args.verbose {
        println!("Total versions: {}", filtered.len());
    }
    println!("Comparing {} versions", filtered.len());
    println!();

    // Compare versions, skipping unchanged ones
    let mut last_changed_idx = 0;

    for i in 1..filtered.len() {
        let old_ver = &filtered[last_changed_idx];
        let new_ver = &filtered[i];

        // Download JARs
        let jar1 = download_jar(
            &agent,
            repo_url,
            &local_repo,
            &group_id,
            &artifact_id,
            old_ver,
            args.verbose,
        )?;
        let jar2 = download_jar(
            &agent,
            repo_url,
            &local_repo,
            &group_id,
            &artifact_id,
            new_ver,
            args.verbose,
        )?;

        if jar1.is_none() || jar2.is_none() {
            if i == 1 {
                // Only show sub-modules hint on first failure
                println!("\nNo JAR files found. Checking for sub-modules...");

                let modules = find_submodules(&agent, repo_url, &group_id, &artifact_id)?;

                if !modules.is_empty() {
                    println!("\nFound {} sub-modules:", modules.len());
                    for (idx, module) in modules.iter().enumerate() {
                        println!("  {}. {}", idx + 1, module);
                    }
                    println!("\nTry one of these:");
                    if let Some(first_module) = modules.first() {
                        println!("  dep_class_diff {}/{}", group_id, first_module);
                    }
                } else {
                    println!("\nThis is a POM-only project with no sub-modules.");
                    println!("Try a different artifact.");
                }
                return Ok(());
            }
            continue;
        }

        // Compare
        let idx1 = index_jar(&jar1.unwrap())?;
        let idx2 = index_jar(&jar2.unwrap())?;

        let (added, removed, modified) = diff(&idx1, &idx2);

        // Skip if no changes
        if added.is_empty() && removed.is_empty() && modified.is_empty() {
            continue;
        }

        // Has changes, print it
        println!("===== {}  ->  {} =====", old_ver, new_ver);

        let limit = if args.full { usize::MAX } else { 10 };

        if !added.is_empty() {
            println!("[ADDED] {}", added.len());
            for c in added.iter().take(limit) {
                println!("  + {}", c);
            }
            if added.len() > limit {
                println!("  ... and {} more", added.len() - limit);
            }
        }
        if !removed.is_empty() {
            println!("[REMOVED] {}", removed.len());
            for c in removed.iter().take(limit) {
                println!("  - {}", c);
            }
            if removed.len() > limit {
                println!("  ... and {} more", removed.len() - limit);
            }
        }
        if !modified.is_empty() {
            println!("[MODIFIED] {}", modified.len());
            if args.full {
                for c in modified.iter().take(limit) {
                    println!("  * {}", c);
                }
                if modified.len() > limit {
                    println!("  ... and {} more", modified.len() - limit);
                }
            }
        }

        println!();

        // Update last changed index
        last_changed_idx = i;
    }

    Ok(())
}

fn parse_artifact(artifact: &str) -> (String, String, bool) {
    // Maven Central Search: https://central.sonatype.com/artifact/org.jeecgframework.boot/jeecg-boot-starter
    if artifact.starts_with("https://central.sonatype.com/artifact/") {
        let path = artifact
            .trim_start_matches("https://central.sonatype.com/artifact/")
            .trim_end_matches('/');

        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return (parts[0].to_string(), parts[1].to_string(), false);
        }
    }

    // Maven Central URL: https://repo1.maven.org/maven2/org/jeecgframework/boot/jeecg-boot-common/
    if artifact.starts_with("https://repo1.maven.org/maven2/")
        || artifact.starts_with("http://repo1.maven.org/maven2/")
    {
        let path = artifact
            .trim_start_matches("https://repo1.maven.org/maven2/")
            .trim_start_matches("http://repo1.maven.org/maven2/")
            .trim_end_matches('/');

        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            let artifact_id = parts[parts.len() - 1];
            let group_parts = &parts[..parts.len() - 1];
            let group_id = group_parts.join(".");
            return (group_id, artifact_id.to_string(), false);
        }
    }

    // GitHub URL: https://github.com/jeecgboot/JeecgBoot or https://github.com/jeecgboot/JeecgBoot/tree/main/...
    if artifact.starts_with("https://github.com/") || artifact.starts_with("http://github.com/") {
        let path = artifact
            .trim_start_matches("https://github.com/")
            .trim_start_matches("http://github.com/")
            .trim_end_matches('/');

        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return (parts[0].to_string(), parts[1].to_string(), true);
        }
    }

    // GitHub: owner/repo (no dots)
    if artifact.contains('/') && !artifact.contains('.') {
        let parts: Vec<&str> = artifact.split('/').collect();
        if parts.len() == 2 {
            return (parts[0].to_string(), parts[1].to_string(), true);
        }
    }

    // Maven: group.id/artifact or group.id:artifact
    let sep = if artifact.contains(':') { ':' } else { '/' };
    let parts: Vec<&str> = artifact.split(sep).collect();
    if parts.len() >= 2 {
        return (parts[0].to_string(), parts[1].to_string(), false);
    }

    (artifact.to_string(), artifact.to_string(), false)
}

fn fetch_versions(
    agent: &Arc<ureq::Agent>,
    repo_url: &str,
    group_id: &str,
    artifact_id: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/{}/{}/maven-metadata.xml",
        repo_url,
        group_id.replace('.', "/"),
        artifact_id
    );

    let resp = agent.get(&url).call()?;
    let xml = resp.into_string()?;

    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut versions = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"version" => {
                if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                    let v = t.unescape()?.trim().to_string();
                    if !v.is_empty() {
                        versions.push(v);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    versions.sort_by(|a, b| version_cmp(a, b));
    Ok(versions)
}

fn filter_versions(versions: &[String], from: Option<&str>, to: Option<&str>) -> Vec<String> {
    versions
        .iter()
        .filter(|v| {
            if let Some(f) = from {
                if version_cmp(v, f) == std::cmp::Ordering::Less {
                    return false;
                }
            }
            if let Some(t) = to {
                if version_cmp(v, t) == std::cmp::Ordering::Greater {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

fn version_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let parts_a: Vec<&str> = a.split(['.', '-']).collect();
    let parts_b: Vec<&str> = b.split(['.', '-']).collect();

    for i in 0..parts_a.len().max(parts_b.len()) {
        let pa = parts_a.get(i).unwrap_or(&"");
        let pb = parts_b.get(i).unwrap_or(&"");

        match (pa.parse::<i64>(), pb.parse::<i64>()) {
            (Ok(na), Ok(nb)) => {
                if na != nb {
                    return na.cmp(&nb);
                }
            }
            _ => {
                let cmp = pa.to_lowercase().cmp(&pb.to_lowercase());
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
        }
    }

    std::cmp::Ordering::Equal
}

fn download_jar(
    agent: &Arc<ureq::Agent>,
    repo_url: &str,
    local_repo: &Path,
    group_id: &str,
    artifact_id: &str,
    version: &str,
    verbose: bool,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let dir = local_repo
        .join(group_id.replace('.', "/"))
        .join(artifact_id)
        .join(version);

    let jar = dir.join(format!("{}-{}.jar", artifact_id, version));

    if jar.exists() {
        if verbose {
            println!("Using cached: {}", version);
        }
        return Ok(Some(jar));
    }

    fs::create_dir_all(&dir)?;

    let url = format!(
        "{}/{}/{}/{}/{}-{}.jar",
        repo_url,
        group_id.replace('.', "/"),
        artifact_id,
        version,
        artifact_id,
        version
    );

    if verbose {
        println!("Downloading: {}", version);
    }

    let resp = agent.get(&url).call();
    match resp {
        Ok(resp) if resp.status() == 200 => {
            let mut reader = resp.into_reader();
            let mut f = fs::File::create(&jar)?;
            io::copy(&mut reader, &mut f)?;
            Ok(Some(jar))
        }
        _ => Ok(None),
    }
}

fn index_jar(jar_path: &Path) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
    let f = fs::File::open(jar_path)?;
    let mut archive = zip::ZipArchive::new(f)?;
    let mut index = HashMap::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.is_dir() {
            continue;
        }

        let name = file.name();
        if !name.ends_with(".class") || name == "module-info.class" {
            continue;
        }

        let class_name = name.trim_end_matches(".class").replace('/', ".");
        let crc = file.crc32();
        let size = file.size();
        let fingerprint = ((crc as u64) << 32) | (size & 0xFFFFFFFF);

        index.insert(class_name, fingerprint);
    }

    Ok(index)
}

fn diff(
    old: &HashMap<String, u64>,
    new: &HashMap<String, u64>,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();

    for (k, new_fp) in new.iter() {
        match old.get(k) {
            None => added.push(k.clone()),
            Some(old_fp) if old_fp != new_fp => modified.push(k.clone()),
            _ => {}
        }
    }

    for k in old.keys() {
        if !new.contains_key(k) {
            removed.push(k.clone());
        }
    }

    added.sort();
    removed.sort();
    modified.sort();

    (added, removed, modified)
}

fn find_submodules(
    agent: &Arc<ureq::Agent>,
    repo_url: &str,
    group_id: &str,
    artifact_id: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Try to list directory on Maven Central
    let base_url = format!("{}/{}/", repo_url, group_id.replace('.', "/"));

    let resp = agent.get(&base_url).call();
    if resp.is_err() {
        return Ok(Vec::new());
    }

    let html = resp.unwrap().into_string()?;
    let mut modules = Vec::new();

    // Parse HTML to find artifact directories
    // Look for links that start with artifact_id prefix
    for line in html.lines() {
        if line.contains("href=\"") {
            if let Some(start) = line.find("href=\"") {
                if let Some(end) = line[start + 6..].find("\"") {
                    let link = &line[start + 6..start + 6 + end];
                    let link = link.trim_end_matches('/');

                    // Check if it's a sub-module (starts with artifact_id)
                    if link.starts_with(artifact_id) && link != artifact_id && !link.contains("..")
                    {
                        modules.push(link.to_string());
                    }
                }
            }
        }
    }

    modules.sort();
    modules.dedup();
    Ok(modules)
}

fn analyze_github(
    owner: &str,
    repo: &str,
    from: Option<&str>,
    to: Option<&str>,
    verbose: bool,
    full: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    let cache_dir = std::env::temp_dir()
        .join("dep_class_diff")
        .join(format!("{}-{}", owner, repo));
    let repo_dir = cache_dir.join("repo");

    // Clone or update repo
    if !repo_dir.exists() {
        println!("Cloning repository...");
        fs::create_dir_all(&cache_dir)?;
        let status = Command::new("git")
            .args([
                "clone",
                "--bare",
                &format!("https://github.com/{}/{}.git", owner, repo),
                repo_dir.to_str().unwrap(),
            ])
            .status()?;
        if !status.success() {
            return Err("Git clone failed".into());
        }
    } else if verbose {
        println!("Using cached repository");
    }

    // Get all tags
    let output = Command::new("git")
        .current_dir(&repo_dir)
        .args(["tag", "-l"])
        .output()?;

    let tags_str = String::from_utf8_lossy(&output.stdout);
    let mut tags: Vec<String> = tags_str.lines().map(|s| s.to_string()).collect();
    tags.sort_by(|a, b| version_cmp(a, b));

    if tags.is_empty() {
        println!("No tags found");
        return Ok(());
    }

    // Filter tags
    let filtered: Vec<String> = tags
        .iter()
        .filter(|t| {
            if let Some(f) = from {
                if version_cmp(t, f) == std::cmp::Ordering::Less {
                    return false;
                }
            }
            if let Some(t_to) = to {
                if version_cmp(t, t_to) == std::cmp::Ordering::Greater {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    if filtered.len() < 2 {
        println!("Need at least 2 tags");
        if !tags.is_empty() {
            println!("Available tags: {} to {}", tags[0], tags[tags.len() - 1]);
        }
        return Ok(());
    }

    if verbose {
        println!("Total tags: {}", filtered.len());
    }
    println!("Comparing {} tags", filtered.len());
    println!();

    // Compare tags, skipping unchanged ones
    let mut last_changed_idx = 0;

    for i in 1..filtered.len() {
        let old_tag = &filtered[last_changed_idx];
        let new_tag = &filtered[i];

        // Extract class names from both tags
        let classes1 = extract_classes_from_tag(&repo_dir, old_tag)?;
        let classes2 = extract_classes_from_tag(&repo_dir, new_tag)?;

        let (added, removed, modified) = diff_classes(&classes1, &classes2);

        // Skip if no changes
        if added.is_empty() && removed.is_empty() && modified.is_empty() {
            continue;
        }

        // Has changes, print it
        println!("===== {}  ->  {} =====", old_tag, new_tag);

        if !added.is_empty() {
            println!("[ADDED] {}", added.len());
            print_grouped_classes(&added, "+", full);
        }
        if !removed.is_empty() {
            println!("[REMOVED] {}", removed.len());
            print_grouped_classes(&removed, "-", full);
        }
        if !modified.is_empty() {
            println!("[MODIFIED] {}", modified.len());
            if full {
                print_grouped_classes(&modified, "*", full);
            }
        }

        println!();

        // Update last changed index
        last_changed_idx = i;
    }

    Ok(())
}

fn extract_classes_from_tag(
    repo_dir: &Path,
    tag: &str,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::process::Command;

    // List all .java files in the tag
    let output = Command::new("git")
        .current_dir(repo_dir)
        .args(["ls-tree", "-r", "--name-only", tag])
        .output()?;

    let files_str = String::from_utf8_lossy(&output.stdout);
    let java_files: Vec<&str> = files_str
        .lines()
        .filter(|f| f.ends_with(".java") && !f.contains("/test/"))
        .collect();

    let mut classes = HashMap::new();

    for file in java_files {
        // Get file content
        let output = Command::new("git")
            .current_dir(repo_dir)
            .args(["show", &format!("{}:{}", tag, file)])
            .output()?;

        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);

            // Calculate hash
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            let hash = hasher.finish();

            // Convert file path to class name
            // Remove common prefixes to make it cleaner
            let mut class_path = file;
            for prefix in &["src/main/java/", "src/", ""] {
                if let Some(stripped) = class_path.strip_prefix(prefix) {
                    class_path = stripped;
                    break;
                }
            }

            let class_name = class_path.trim_end_matches(".java").replace('/', ".");

            classes.insert(class_name, format!("{:x}", hash));
        }
    }

    Ok(classes)
}

fn diff_classes(
    old: &HashMap<String, String>,
    new: &HashMap<String, String>,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();

    for (k, new_hash) in new.iter() {
        match old.get(k) {
            None => added.push(k.clone()),
            Some(old_hash) if old_hash != new_hash => modified.push(k.clone()),
            _ => {}
        }
    }

    for k in old.keys() {
        if !new.contains_key(k) {
            removed.push(k.clone());
        }
    }

    added.sort();
    removed.sort();
    modified.sort();

    (added, removed, modified)
}

fn print_grouped_classes(classes: &[String], prefix: &str, full: bool) {
    use std::collections::BTreeMap;

    // Group by module path (everything before the last package segment)
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for class in classes {
        // Find the module path (e.g., "jeecg-boot-base-core.src.main.java")
        // and the class path (e.g., "org.jeecg.common.system.query.QueryGenerator")
        if let Some(pos) = class.find(".org.") {
            let module = &class[..pos];
            let class_name = &class[pos + 1..]; // Skip the dot
            groups
                .entry(module.to_string())
                .or_default()
                .push(class_name.to_string());
        } else if let Some(pos) = class.find(".com.") {
            let module = &class[..pos];
            let class_name = &class[pos + 1..];
            groups
                .entry(module.to_string())
                .or_default()
                .push(class_name.to_string());
        } else {
            // Fallback: no clear module separation
            groups.entry(String::new()).or_default().push(class.clone());
        }
    }

    let mut total_shown = 0;
    let limit = if full { usize::MAX } else { 50 };

    for (module, mut class_list) in groups {
        if total_shown >= limit {
            break;
        }

        class_list.sort();

        if !module.is_empty() {
            println!("  {}:", module);
        }

        for class_name in class_list {
            if total_shown >= limit {
                break;
            }
            if module.is_empty() {
                println!("  {} {}", prefix, class_name);
            } else {
                println!("    {} {}", prefix, class_name);
            }
            total_shown += 1;
        }
    }

    if classes.len() > limit {
        println!("  ... and {} more", classes.len() - limit);
    }
}

fn local_m2_repo() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE"))?;
    Ok(PathBuf::from(home).join(".m2").join("repository"))
}
