# 快速开始指南

## 安装

### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/baiqll/dep_class_diff/main/install.sh | bash
```

### Windows

从 [Releases](https://github.com/baiqll/dep_class_diff/releases) 下载 `dep_class_diff-x86_64-pc-windows-msvc.exe`，重命名为 `dep_class_diff.exe` 并添加到 PATH。

## 基本使用

### 1. 分析 Maven 项目

```bash
# 最简单的方式 - 直接粘贴 Maven Central URL
dep_class_diff https://central.sonatype.com/artifact/commons-io/commons-io

# 或使用简短格式
dep_class_diff commons-io/commons-io

# 指定版本范围
dep_class_diff commons-io/commons-io 2.11.0 2.16.0
```

### 2. 分析 GitHub 项目

```bash
# 直接粘贴 GitHub URL
dep_class_diff https://github.com/apache/commons-io

# 指定标签范围
dep_class_diff apache/commons-io rel/commons-io-2.11.0 rel/commons-io-2.16.0
```

### 3. 查看详细信息

```bash
# 显示所有 MODIFIED 类的详细列表
dep_class_diff commons-io/commons-io 2.11.0 2.16.0 --full

# 查看版本信息
dep_class_diff commons-io/commons-io -v
```

## 输出说明

```
===== 2.11.0  ->  2.12.0 =====
[ADDED] 8          # 新增的类
  + org.apache.commons.io.file.Counters
  + org.apache.commons.io.file.PathUtils
  ...

[REMOVED] 2        # 删除的类
  - org.apache.commons.io.deprecated.OldClass
  ...

[MODIFIED] 15      # 修改的类（默认只显示数量）
```

使用 `--full` 参数可以看到 MODIFIED 类的详细列表。

## 常见问题

### Q: 为什么有些版本找不到？
A: Maven Central 上可能没有所有版本的 JAR 文件，特别是一些老版本或 POM-only 项目。

### Q: GitHub 模式和 Maven 模式有什么区别？
A: 
- Maven 模式：分析编译后的 JAR 文件中的 .class 文件（更精确）
- GitHub 模式：分析源码中的 .java 文件（可能包含未发布的代码）

### Q: 如何处理 POM-only 项目？
A: 工具会自动检测并列出所有子模块，你可以选择其中一个进行分析。

## 更多示例

查看 [README.md](README.md) 获取更多详细示例和说明。

## 获取帮助

```bash
dep_class_diff --help
```

或访问 [GitHub Issues](https://github.com/baiqll/dep_class_diff/issues)。
