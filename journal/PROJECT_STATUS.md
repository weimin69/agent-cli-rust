# PROJECT STATUS

## Current Stage

当前项目处于 Rust CLI 基础架构学习阶段。

目标是通过一个小型 Agent CLI 项目，逐步掌握：

- Rust 基础语法
- CLI 工程结构
- 模块系统
- 命令分发
- 后续 Agent 能力所需的配置、错误处理、异步请求与 API 调用

当前重点不是快速实现 Agent，而是先建立清晰、可扩展的 CLI 架构，并开始引入错误处理思维。

## Completed

- 创建 Cargo Rust 项目
- 初始化 GitHub 仓库
- 集成 `clap`
- 使用 `Parser` 定义 CLI 入口
- 使用 `Subcommand` 定义子命令
- 实现 `hello` 命令
- 实现 `version` 命令
- 实现 `echo` 命令
- 为 `echo` 增加必填参数约束
- 为 `echo` 增加 `--upper` flag
- 将 `echo::execute` 调整为接收 `&[String]`
- 使用 `cargo fmt` 统一代码格式
- 实现 `divide` 命令
- 在 `divide` 中处理除以 0 的基础错误输出
- 将 CLI 定义拆分到 `src/cli.rs`
- 将命令实现拆分到 `src/commands/`
- 为每个命令建立独立模块
- 使用 `src/commands/mod.rs` 统一导出命令模块
- 保持 `src/main.rs` 只负责解析参数和命令分发
- 初步理解 Rust 模块系统
- 初步理解 `Vec<String>` 在 CLI 参数中的作用
- 初步理解所有权移动
- 初步理解借用和 slice 参数
- 初步理解 CLI 参数校验和业务校验的边界

## In Progress

围绕 `divide` 命令准备进入 Rust 错误处理：

- `Result<T, E>`
- `Ok`
- `Err`
- 错误返回
- `main.rs` 中的错误处理

当前 `divide` 已经可以处理正常除法和除以 0 场景，但错误仍然是在命令内部直接打印。下一步要把错误返回给上层处理。

## Next Step

下一步从 `divide` 命令开始，重点理解：

- 为什么 `execute()` 应该返回 `Result<(), String>`
- `Ok(())` 表示什么
- `Err(String)` 表示什么
- 为什么错误不应该总是在业务函数内部直接打印
- `main.rs` 如何用 `if let Err(error)` 处理错误

完成这些理解后，再进入：

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
│   ├── divide.rs
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

- `divide::execute()` 应该返回什么错误类型？
- 什么时候使用 `String` 作为错误，什么时候引入 `anyhow`？
- 后续命令越来越多时，是否需要改进 `main.rs` 中的分发方式？
- `main()` 是否应该改为返回 `Result<()>`？

## Technical Debt

- 当前命令没有测试
- 当前项目还没有统一错误处理模型
- 当前日志文件命名存在 `2026-7-13.md`、`2026-7-14.md`，后续建议统一为 `YYYY-MM-DD.md`
- 旧日志文件 `2026-7-14.md` 与标准命名 `2026-07-14.md` 同时存在，后续需要决定是否迁移或保留
