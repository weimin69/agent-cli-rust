# PROJECT STATUS

## Current Stage

当前项目处于 Rust CLI 基础架构学习阶段。

目标是通过一个小型 Agent CLI 项目，逐步掌握：

- Rust 基础语法
- CLI 工程结构
- 模块系统
- 命令分发
- 后续 Agent 能力所需的配置、错误处理、异步请求与 API 调用

当前重点不是快速实现 Agent，而是先建立清晰、可扩展的 CLI 架构。

## Completed

- 创建 Cargo Rust 项目
- 初始化 GitHub 仓库
- 集成 `clap`
- 使用 `Parser` 定义 CLI 入口
- 使用 `Subcommand` 定义子命令
- 实现 `hello` 命令
- 实现 `version` 命令
- 实现 `echo` 命令
- 将 CLI 定义拆分到 `src/cli.rs`
- 将命令实现拆分到 `src/commands/`
- 为每个命令建立独立模块
- 使用 `src/commands/mod.rs` 统一导出命令模块
- 保持 `src/main.rs` 只负责解析参数和命令分发
- 初步理解 Rust 模块系统
- 初步理解 `Vec<String>` 在 CLI 参数中的作用

## In Progress

围绕 `echo` 命令继续学习 Rust 基础概念：

- `Vec<String>`
- Ownership
- Borrowing
- `String` 与 `&str`

当前已有代码可以作为后续学习所有权和借用的基础。

## Next Step

下一步从 `echo` 命令开始，重点理解：

- `words: Vec<String>` 表示什么
- 为什么 `execute(words: Vec<String>)` 会取得所有权
- 什么情况下应该传 `String`
- 什么情况下应该传 `&str`
- 什么情况下应该传 `&[String]`

完成这些理解后，再进入：

- `Result`
- Error Handling
- Config
- `serde`
- `reqwest`
- `async`
- OpenAI API
- Agent loop

## Architecture Notes

当前项目结构：

```text
src
├── commands
│   ├── echo.rs
│   ├── hello.rs
│   ├── version.rs
│   └── mod.rs
├── cli.rs
└── main.rs
```

当前职责划分：

- `src/cli.rs`：定义 CLI 结构和子命令，不写业务逻辑
- `src/commands/`：每个命令一个文件，负责具体业务逻辑
- `src/commands/mod.rs`：统一导出命令模块
- `src/main.rs`：负责 `Cli::parse()`、`match Commands` 和调用对应 `execute()`

新增命令时应遵循：

1. 在 `src/cli.rs` 中定义命令参数
2. 在 `src/commands/` 下新增命令模块
3. 在 `src/commands/mod.rs` 中导出模块
4. 在 `src/main.rs` 中添加命令分发
5. 在命令模块中提供 `execute()` 函数

## Open Questions

- `execute()` 函数参数应该使用 owned value 还是 borrowed value？
- `echo` 命令是否应该支持空参数？
- 后续命令越来越多时，是否需要改进 `main.rs` 中的分发方式？
- 什么时候开始引入 `Result` 作为统一错误处理方式？

## Technical Debt

- 需要运行 `cargo fmt` 统一代码格式
- `src/cli.rs` 中空格、空行和逗号风格需要整理
- `src/main.rs` 中 `match` 分支格式可以更统一
- 当前命令没有测试
- 当前项目还没有统一错误处理模型
- 当前日志文件命名存在 `2026-7-13.md`、`2026-7-14.md`，后续建议统一为 `YYYY-MM-DD.md`
