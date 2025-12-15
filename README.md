# Class Diff - Maven 版本差异分析工具

[![CI](https://github.com/baiqll/dep_class_diff/actions/workflows/ci.yml/badge.svg)](https://github.com/baiqll/dep_class_diff/actions/workflows/ci.yml)
[![Release](https://github.com/baiqll/dep_class_diff/actions/workflows/release.yml/badge.svg)](https://github.com/baiqll/dep_class_diff/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

快速分析 Maven Central 上的依赖不同版本之间的 class 文件变化。

## 安装

### 方式 1: 使用安装脚本（推荐）

```bash
curl -fsSL https://raw.githubusercontent.com/baiqll/dep_class_diff/main/install.sh | bash
```

### 方式 2: 从 Release 下载

访问 [Releases](https://github.com/baiqll/dep_class_diff/releases) 页面下载对应平台的二进制文件。

支持的平台：
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64/Apple Silicon)
- Windows (x86_64)

### 方式 3: 从源码编译

```bash
git clone https://github.com/baiqll/dep_class_diff.git
cd dep_class_diff
cargo build --release
# 二进制文件位于 target/release/dep_class_diff
```

## 使用

支持多种格式，直接粘贴 URL 即可：

```bash
# 方式 1: 简短格式
dep_class_diff commons-io/commons-io

# 方式 2: Maven Central 搜索页面 URL（直接复制浏览器地址）
dep_class_diff https://central.sonatype.com/artifact/commons-io/commons-io

# 方式 3: Maven 仓库 URL
dep_class_diff https://repo1.maven.org/maven2/commons-io/commons-io/

# 方式 4: 自定义 Maven 仓库 URL（直接粘贴完整路径）
dep_class_diff https://maven.jeecg.org/nexus/content/repositories/jeecg/com/jimureport/spring-boot-starter-jimureport/

# 方式 5: 使用 -r 参数指定自定义仓库
dep_class_diff com.jimureport/spring-boot-starter-jimureport -r https://maven.jeecg.org/nexus/content/repositories/jeecg

# 方式 6: GitHub URL
dep_class_diff https://github.com/apache/commons-io



# 指定版本范围
dep_class_diff commons-io/commons-io 2.11.0 2.16.0

# 查看详细信息
dep_class_diff commons-io/commons-io -v

# 显示全部内容（包括 MODIFIED 详细列表）
dep_class_diff https://github.com/apache/commons-io rel/commons-io-2.11.0 rel/commons-io-2.16.0 --full
```

就这么简单！直接复制粘贴 URL 就能用。

## 说明

- **智能对比**：自动跳过无变化的版本，只显示有差异的对比
- **支持 Maven Central、自定义 Maven 仓库和 GitHub**
- **自定义仓库**：支持任何公开的 Maven 仓库（如企业私服、Nexus、Artifactory 等）
- 使用 `-v` 查看可用版本数量
- 使用 `-r` 指定自定义 Maven 仓库 URL
- 使用 `--full` 或 `-f` 显示全部内容（默认 MODIFIED 只显示数量，不显示详细列表）
- **POM 项目自动处理**：如果是 POM-only 项目，会自动列出所有子模块供选择

### 工作原理

工具会逐个比较版本，如果某个版本没有变化，会自动跳过并与下一个有变化的版本对比。

例如：v1→v2 有变化，v2→v3 无变化，v3→v4 有变化，则显示：
- v1 → v2 (变化详情)
- v2 → v4 (变化详情，自动跳过 v3)

## GitHub 项目支持

现在也支持 GitHub 项目了！通过分析源码中的 Java 类：

```bash
# GitHub 项目
dep_class_diff https://github.com/apache/commons-io rel/commons-io-2.11.0 rel/commons-io-2.16.0

# 或简短格式
dep_class_diff apache/commons-io rel/commons-io-2.11.0 rel/commons-io-2.16.0
```

**注意**：
- GitHub 模式分析的是源码中的类名（不是编译后的 class）
- 第一次会克隆仓库，后续使用缓存
- 需要安装 git 命令
- GitHub 模式会按模块路径分组显示（如 `src.main.java:`）

## 不支持的功能

- ❌ 需要认证的私有仓库（Maven 和 GitHub 都不支持）
- ✅ 公开的自定义 Maven 仓库已支持

## 输出示例

### Maven 项目示例
```
$ dep_class_diff commons-io/commons-io 2.11.0 2.16.0

Comparing 6 versions

===== 2.11.0  ->  2.12.0 =====
[ADDED] 8
  + org.apache.commons.io.file.Counters
  + org.apache.commons.io.file.Counters$Counter
  + org.apache.commons.io.file.PathUtils
  + org.apache.commons.io.function.IOBiConsumer
  + org.apache.commons.io.function.IOConsumer
  + org.apache.commons.io.function.IOFunction
  + org.apache.commons.io.function.IOSupplier
  + org.apache.commons.io.input.BrokenReader
[MODIFIED] 15

===== 2.12.0  ->  2.13.0 =====
[ADDED] 5
  + org.apache.commons.io.file.AccumulatorPathVisitor
  + org.apache.commons.io.file.CountingPathVisitor
  + org.apache.commons.io.file.DeletingPathVisitor
  + org.apache.commons.io.file.PathFilter
  + org.apache.commons.io.input.MessageDigestInputStream
[MODIFIED] 12

===== 2.13.0  ->  2.16.0 =====
(注意：2.14.0, 2.15.0 无变化，自动跳过)
[ADDED] 3
  + org.apache.commons.io.input.UnsynchronizedBufferedInputStream
  + org.apache.commons.io.output.UnsynchronizedByteArrayOutputStream
  + org.apache.commons.io.output.WriterOutputStream
[MODIFIED] 8

# 使用 --full 查看 MODIFIED 详细列表
$ dep_class_diff commons-io/commons-io 2.11.0 2.16.0 --full

===== 2.11.0  ->  2.12.0 =====
[MODIFIED] 15
  * org.apache.commons.io.FileUtils
  * org.apache.commons.io.IOUtils
  * org.apache.commons.io.file.PathUtils
  * org.apache.commons.io.input.BOMInputStream
  * org.apache.commons.io.input.ReaderInputStream
  * org.apache.commons.io.output.ByteArrayOutputStream
  * org.apache.commons.io.output.FileWriterWithEncoding
  * org.apache.commons.io.output.StringBuilderWriter
  ... and 7 more
```

### POM 项目示例（自动列出所有子模块）
```
$ dep_class_diff org.apache.commons/commons-parent

No JAR files found. Checking for sub-modules...

Found 15 sub-modules:
  1. commons-parent-1
  2. commons-parent-2
  3. commons-parent-3
  4. commons-parent-4
  5. commons-parent-5
  ... (显示全部 15 个)

Try one of these:
  dep_class_diff org.apache.commons/commons-parent-1
```

### GitHub 项目示例（按模块分组）
```
$ dep_class_diff https://github.com/apache/commons-io rel/commons-io-2.11.0 rel/commons-io-2.16.0

Comparing 6 tags

===== rel/commons-io-2.11.0  ->  rel/commons-io-2.12.0 =====
[ADDED] 18
  src.main.java:
    + org.apache.commons.io.file.Counters
    + org.apache.commons.io.file.Counters$Counter
    + org.apache.commons.io.file.Counters$PathCounters
    + org.apache.commons.io.file.PathUtils
    + org.apache.commons.io.function.IOBiConsumer
    + org.apache.commons.io.function.IOConsumer
    + org.apache.commons.io.function.IOFunction
    + org.apache.commons.io.function.IOSupplier
    + org.apache.commons.io.input.BrokenReader
    + org.apache.commons.io.input.CharSequenceReader
  ... and 8 more

[MODIFIED] 25

# 使用 --full 查看 MODIFIED 详细列表
$ dep_class_diff https://github.com/apache/commons-io rel/commons-io-2.11.0 rel/commons-io-2.16.0 --full

===== rel/commons-io-2.11.0  ->  rel/commons-io-2.12.0 =====
[MODIFIED] 25
  src.main.java:
    * org.apache.commons.io.FileUtils
    * org.apache.commons.io.IOUtils
    * org.apache.commons.io.file.PathUtils
    * org.apache.commons.io.input.BOMInputStream
    * org.apache.commons.io.input.ReaderInputStream
    * org.apache.commons.io.output.ByteArrayOutputStream
    * org.apache.commons.io.output.FileWriterWithEncoding
  ... and 18 more
```

## License

MIT

## 开发

### 构建

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 代码检查
cargo clippy

# 格式化
cargo fmt
```

### 发布新版本

```bash
# 创建新版本（会自动更新 Cargo.toml、创建 tag 并推送）
./scripts/release.sh v0.1.0
```

GitHub Actions 会自动构建所有平台的二进制文件并创建 Release。

## 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 相关项目

- [Maven Central](https://central.sonatype.com/) - Maven 中央仓库
- [Apache Commons IO](https://github.com/apache/commons-io) - 示例项目
