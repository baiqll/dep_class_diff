# GitHub 项目设置指南

本文档说明如何将项目上传到 GitHub 并配置自动化。

## 📁 已创建的文件

### 核心文件
- `install.sh` - 自动安装脚本（支持 Linux/macOS）
- `LICENSE` - MIT 许可证
- `CONTRIBUTING.md` - 贡献指南
- `QUICKSTART.md` - 快速开始指南

### GitHub Actions 工作流
- `.github/workflows/ci.yml` - 持续集成（每次提交时运行测试）
- `.github/workflows/release.yml` - 自动发布（创建 tag 时自动构建所有平台）
- `.github/workflows/dependencies.yml` - 自动更新依赖（每周一运行）

### GitHub 模板
- `.github/ISSUE_TEMPLATE/bug_report.md` - Bug 报告模板
- `.github/ISSUE_TEMPLATE/feature_request.md` - 功能请求模板
- `.github/RELEASE_TEMPLATE.md` - 发布说明模板

### 脚本
- `scripts/release.sh` - 发布脚本（自动化版本发布流程）

## 🚀 上传到 GitHub

### 1. 创建 GitHub 仓库

访问 https://github.com/new 创建新仓库，例如：
- 仓库名：`dep_class_diff`
- 描述：`快速分析 Maven/GitHub 项目不同版本之间的 class 文件变化`
- 公开/私有：根据需要选择
- 不要初始化 README（我们已经有了）

### 2. 更新文件中的占位符

在以下文件中将 `baiqll` 替换为你的 GitHub 用户名：

```bash
# 使用 sed 批量替换（macOS）
find . -type f \( -name "*.md" -o -name "*.sh" -o -name "*.yml" \) -exec sed -i '' 's/baiqll/你的用户名/g' {} +

# 或者使用 sed 批量替换（Linux）
find . -type f \( -name "*.md" -o -name "*.sh" -o -name "*.yml" \) -exec sed -i 's/baiqll/你的用户名/g' {} +
```

需要替换的文件：
- `README.md`
- `install.sh`
- `CONTRIBUTING.md`
- `QUICKSTART.md`
- `scripts/release.sh`
- `.github/RELEASE_TEMPLATE.md`

### 3. 初始化 Git 并推送

```bash
# 初始化 Git（如果还没有）
git init

# 添加所有文件
git add .

# 提交
git commit -m "Initial commit"

# 添加远程仓库（替换 baiqll）
git remote add origin https://github.com/baiqll/dep_class_diff.git

# 推送到 GitHub
git branch -M main
git push -u origin main
```

## 📦 发布第一个版本

### 方式 1: 使用发布脚本（推荐）

```bash
./scripts/release.sh v0.1.0
```

这会自动：
1. 更新 `Cargo.toml` 中的版本号
2. 更新 `Cargo.lock`
3. 提交更改
4. 创建 git tag
5. 推送到 GitHub

### 方式 2: 手动发布

```bash
# 更新 Cargo.toml 中的版本号
# version = "0.1.0"

# 提交
git add Cargo.toml
git commit -m "chore: bump version to v0.1.0"

# 创建 tag
git tag -a v0.1.0 -m "Release v0.1.0"

# 推送
git push origin main
git push origin v0.1.0
```

### 3. GitHub Actions 自动构建

推送 tag 后，GitHub Actions 会自动：
1. 构建所有平台的二进制文件：
   - Linux (x86_64, aarch64)
   - macOS (x86_64, aarch64)
   - Windows (x86_64)
2. 创建 GitHub Release
3. 上传所有二进制文件

访问 `https://github.com/baiqll/dep_class_diff/actions` 查看构建进度。

## ✅ 验证设置

### 1. 检查 CI 工作流

提交代码后，访问 Actions 页面确认 CI 通过：
- ✅ 格式检查
- ✅ Clippy 检查
- ✅ 构建成功
- ✅ 测试通过

### 2. 检查 Release 工作流

创建 tag 后，确认：
- ✅ 所有平台构建成功
- ✅ Release 自动创建
- ✅ 二进制文件已上传

### 3. 测试安装脚本

```bash
# 测试安装脚本（替换 baiqll）
curl -fsSL https://raw.githubusercontent.com/baiqll/dep_class_diff/main/install.sh | bash

# 验证安装
dep_class_diff --help
```

## 🎯 后续步骤

1. **添加项目描述**：在 GitHub 仓库设置中添加描述和标签
2. **启用 Discussions**：在仓库设置中启用讨论功能
3. **添加 Topics**：添加相关标签如 `rust`, `maven`, `java`, `cli-tool`
4. **创建 Wiki**：添加更详细的文档
5. **设置 GitHub Pages**：如果需要项目网站

## 📝 维护

### 更新依赖

依赖会每周一自动检查更新，或手动运行：

```bash
# 本地更新
cargo update

# 或触发 GitHub Actions
# 访问 Actions -> Update Dependencies -> Run workflow
```

### 发布新版本

```bash
# 使用发布脚本
./scripts/release.sh v0.2.0
```

## 🔧 故障排除

### Actions 失败

1. 检查 Actions 日志
2. 确认所有测试在本地通过
3. 检查 Rust 版本兼容性

### 安装脚本失败

1. 确认 Release 已创建
2. 检查二进制文件名称是否正确
3. 验证 URL 中的用户名是否正确

## 📚 相关资源

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Rust 发布最佳实践](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [语义化版本](https://semver.org/lang/zh-CN/)
