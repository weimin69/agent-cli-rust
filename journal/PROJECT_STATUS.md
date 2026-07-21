# PROJECT STATUS

## Current Stage

当前项目处于 Rust CLI 基础架构学习阶段。

目标是通过一个小型 Agent CLI 项目，逐步掌握：

- Rust 基础语法
- CLI 工程结构
- 模块系统
- 命令分发
- 后续 Agent 能力所需的配置、错误处理、异步请求与 API 调用

当前重点不是快速实现 Agent，而是先建立清晰、可扩展的 CLI 架构，并逐步完善错误处理模型。当前已经完成 `run()` / `main()` 职责拆分，将命令错误模型从 `Result<(), String>` 迁移到 `anyhow::Result<()>`，并通过 `read-config` 命令开始练习 `.context()` 和 `with_context()`。

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
- 将 `divide::execute()` 改为返回 `Result<(), String>`
- 将 `echo::execute()`、`hello::execute()`、`version::execute()` 统一改为返回 `Result<(), String>`
- 初步理解 `Ok(())` 和 `Err(String)`
- 初步理解 `if let Err(error)` 的错误处理方式
- 初步理解 CLI 失败时应返回非 0 exit code
- 初步理解 `std::process::exit(1)` 的作用和局限
- 将 `main()` 改为返回 `Result<(), String>`
- 初步使用 `?` 运算符转发错误
- 初步理解“业务函数返回错误，入口层决定如何处理错误”的职责边界
- 实现 `sum` 命令
- 在 `sum::execute()` 中使用 `&[f64]` 只读借用参数
- 验证 `sum` 正常输入、空参数和非法数字参数的行为
- 初步理解 `main()`、`Cli::parse()`、`match` 和命令 `execute()` 之间的调用链
- 初步讨论 `run() -> Result<(), String>` 作为业务入口函数的作用
- 将主要业务入口拆分到 `run() -> Result<(), String>`
- 让 `main()` 调用 `run()`，并使用 `if let Err(error)` 统一处理错误
- 使用 `eprintln!` 将业务错误输出到 `stderr`
- 使用 `std::process::exit(1)` 保持业务错误时返回非 0 exit code
- 初步理解 `stdout` 和 `stderr` 的区别
- 初步理解 `>` 重定向 `stdout`，`2>` 重定向 `stderr`
- 进一步理解 `?` 运算符遇到 `Ok` 会继续执行，遇到 `Err` 会提前返回当前函数
- 修正 `sum` 命令错误文案中的拼写问题
- 实现 `repeat` 命令
- 为 `repeat` 增加 `--times` named argument
- 为 `repeat` 的 `words` 参数增加必填约束
- 在 `repeat::execute()` 中校验 `times == 0` 的业务错误
- 验证 `repeat` 正常输入、业务错误和缺少参数时的行为
- 讨论 `String` 错误模型的优点和局限
- 初步理解 `anyhow` 适合 CLI 应用层错误处理
- 初步理解 `anyhow` 不一定适合库公共 API
- 添加 `anyhow` 依赖
- 将 `run()` 改为返回 `anyhow::Result<()>`
- 将所有命令的 `execute()` 改为返回 `anyhow::Result<()>`
- 使用 `bail!` 表达业务错误
- 验证 `anyhow` 迁移后的正常路径、业务错误路径和 Clap 错误路径
- 实现 `read-config` 命令
- 为 `read-config` 增加路径参数
- 在 `read_config::execute(path: &str)` 中使用只读字符串借用
- 使用 `.context()` 给文件读取失败补充错误上下文
- 初步理解 `with_context()` 适合动态错误上下文
- 验证 `read-config config.toml` 和 `read-config Cargo.toml` 的成功路径
- 验证 `read-config missing.toml` 的失败路径
- 进一步理解相对路径基于程序启动时的当前工作目录

## In Progress

继续围绕 CLI 错误处理和工程职责划分推进：

- `Result<T, E>`
- `Ok`
- `Err`
- 错误返回
- `?` 运算符
- `anyhow::Result<()>`
- `bail!`
- `.context()`
- `with_context()`
- `main()`
- `stdout` / `stderr`
- Clap 参数校验和业务校验边界

当前所有命令的 `execute()` 已统一返回 `anyhow::Result<()>`。`run()` 负责解析 CLI、匹配子命令并使用 `?` 转发业务错误；`main()` 负责统一打印 `error: ...` 并返回失败退出码。`read-config` 已能接收路径参数并读取文件，下一步适合继续完善 `with_context()`，让错误信息包含具体路径。

## Next Step

下一步建议继续基础错误处理模型的第二轮学习收尾：

- Error Handling
- `bail!` 的使用场景
- `?` 和 `bail!` 的区别
- `.context()` 如何给底层错误补充上下文
- `with_context()` 如何生成包含变量的动态上下文
- 如何保持命令模块、`run()` 和 `main()` 的职责边界清晰

完成这些理解后，再进入：

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
│   ├── read_config.rs
│   ├── repeat.rs
│   ├── sum.rs
│   ├── version.rs
│   └── mod.rs
├── cli.rs
└── main.rs
```

当前职责划分：

- `src/cli.rs`：定义 CLI 结构和子命令，不写业务逻辑
- `src/commands/`：每个命令一个文件，负责具体业务逻辑，并通过 `anyhow::Result<()>` 返回执行结果
- `src/commands/mod.rs`：统一导出命令模块
- `src/main.rs`：`run()` 负责 `Cli::parse()`、`match Commands`、调用对应 `execute()`，并通过 `?` 转发错误；`main()` 负责调用 `run()`、统一打印错误和设置失败退出码
- `read-config` 当前用于错误处理练习，负责读取指定路径的文件内容

新增命令时应遵循：

1. 在 `src/cli.rs` 中定义命令参数
2. 在 `src/commands/` 下新增命令模块
3. 在 `src/commands/mod.rs` 中导出模块
4. 在 `src/main.rs` 中添加命令分发
5. 在命令模块中提供返回 `anyhow::Result<()>` 的 `execute()` 函数

## Open Questions

- `bail!`、`?`、`.context()` 在实际代码里如何选择？
- `with_context()` 中如何正确放入路径等动态变量？
- 后续命令越来越多时，是否需要改进 `main.rs` 中的分发方式？
- 什么时候应该继续使用 `?`，什么时候应该手写错误处理？
- 后续是否需要为命令执行结果和错误输出增加自动化测试？

## Technical Debt

- 当前命令没有测试
- `read_config.rs` 当前使用了 `with_context()`，但错误信息还没有包含具体 `path`
- 当前日志文件命名存在 `2026-7-13.md`、`2026-7-14.md`，后续建议统一为 `YYYY-MM-DD.md`
- 旧日志文件 `2026-7-14.md` 与标准命名 `2026-07-14.md` 同时存在，后续需要决定是否迁移或保留

## Next TODO

- [ ] 修正 `read_config.rs` 的 `with_context()`，让错误信息包含具体路径
- [ ] 对比 `.context()` 和 `with_context()` 的使用场景
- [ ] 准备进入简单 config 读取和解析练习
