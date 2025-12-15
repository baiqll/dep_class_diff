# Contributing to dep_class_diff

感谢你考虑为 dep_class_diff 做贡献！

## 开发环境

### 前置要求

- Rust 1.70 或更高版本
- Git

### 设置开发环境

```bash
# 克隆仓库
git clone https://github.com/baiqll/dep_class_diff.git
cd dep_class_diff

# 构建项目
cargo build

# 运行测试
cargo test

# 运行 clippy 检查
cargo clippy

# 格式化代码
cargo fmt
```

## 提交代码

1. Fork 这个仓库
2. 创建你的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的改动 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个 Pull Request

## 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 确保所有测试通过 (`cargo test`)
- 为新功能添加测试
- 更新相关文档

## 报告 Bug

如果你发现了 bug，请在 [Issues](https://github.com/baiqll/dep_class_diff/issues) 页面创建一个新的 issue，并包含：

- 问题描述
- 复现步骤
- 预期行为
- 实际行为
- 系统信息（操作系统、Rust 版本等）

## 功能请求

如果你有新功能的想法，欢迎在 [Issues](https://github.com/baiqll/dep_class_diff/issues) 页面提出。

## 许可证

通过贡献代码，你同意你的贡献将在 MIT 许可证下发布。
