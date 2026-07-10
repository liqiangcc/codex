# 阶段 2：核心源码链路

## 目标与证据

本阶段回答一个问题：一次用户输入如何从 CLI 进入 core，变成模型请求和工具调用，最后把结果写回会话历史？以下结论均来自当前分支源码；行为层证据来自阶段 1 已记录的 `codex exec` JSONL、权限和 TUI 命令观察。

## 端到端链路

```text
CLI 参数
  -> cli_main
  -> TUI / exec 入口
  -> session handler 接收 UserTurn
  -> TurnInput + RegularTask
  -> run_turn
  -> history + context + tools 构造 Prompt
  -> ModelClientSession 流式响应
  -> ToolRouter / ToolOrchestrator
  -> tool output 写入 history
  -> 下一次 sampling，或 assistant message 结束 turn
```

### 1. CLI 分流

- [`codex-rs/cli/src/main.rs`](../codex-rs/cli/src/main.rs) 的 `main` 调用 `cli_main`，解析 `MultitoolCli`。
- 没有子命令时，`cli_main` 调用 `run_interactive_tui`；`exec` 子命令则调用 `codex_exec::run_main`。
- 结论：TUI 与非交互模式有不同的外层入口，但都要建立相同的 core 会话/turn 模型；阶段 1 中两种模式的差异不应被误解为两套 agent 内核。

### 2. 用户输入变成任务

- [`codex-rs/core/src/session/handlers.rs`](../codex-rs/core/src/session/handlers.rs) 处理 `Op::UserTurn`，建立 `TurnContext`，先尝试把输入作为 steer 投入活动 turn。
- 没有活动 turn 时，handler 把附加上下文变成 `TurnInput::ResponseItem`，把用户内容变成 `TurnInput::UserInput`，再调用 `Session::spawn_task` 和 `RegularTask::new()`。
- [`codex-rs/core/src/session/input_queue.rs`](../codex-rs/core/src/session/input_queue.rs) 保存活动 turn 的 pending input，因此 UI 在模型工作期间输入的新消息不必丢失。

不变量：用户输入不是直接调用模型；它必须先拥有 turn context，并被记录为可重放的 `TurnInput`。

### 3. 常规 turn 与模型采样

- [`codex-rs/core/src/tasks/regular.rs`](../codex-rs/core/src/tasks/regular.rs) 的 `RegularTask::run` 先发送 `TurnStarted`，然后循环调用 `run_turn`；如果 input queue 有新输入，会在同一个 task 中继续。
- [`codex-rs/core/src/session/turn.rs`](../codex-rs/core/src/session/turn.rs) 的 `run_turn` 会先检查 pre-sampling compaction，捕获 `StepContext`，记录 context 更新和技能/plugin 注入，再从 history 构建 sampling 输入。
- `run_sampling_request` 为当前 step 创建 tools router、构造 `Prompt`，并复用 turn-scoped `ModelClientSession` 处理可重试的流式请求。
- `try_run_sampling_request` 读取响应流；assistant message 可结束 turn，function/tool call 会被处理后促成下一次 sampling。

不变量：模型看到的输入来自 session history 的一个明确快照；tools、context 和当前 step 共享同一个请求视图。

### 4. 工具、审批与 sandbox

- [`codex-rs/core/src/tools/router.rs`](../codex-rs/core/src/tools/router.rs) 将 `ToolCall` 交给 registry 分发到具体 handler/runtime。
- [`codex-rs/core/src/tools/orchestrator.rs`](../codex-rs/core/src/tools/orchestrator.rs) 集中处理 approval、sandbox 选择、network approval 和被拒后的升级重试。
- 编排顺序是：判断 approval requirement → 选择首次 sandbox → 执行 → 在允许条件下处理拒绝或升级；模型 prompt 不能绕过这条运行时路径。

阶段 1 的 read-only/workspace-write 对照与此一致：同一写请求在 read-only 被拒，在 workspace-write 才能到达写入 runtime。

### 5. 历史压缩与 MCP

- [`codex-rs/core/src/compact.rs`](../codex-rs/core/src/compact.rs) 在 compact 后保留用户消息和摘要，按规则重新注入 initial context，替换 history 并重新计算 token 使用；长线程的精度风险会作为 warning 发出。
- `run_turn` 在 token limit 或新的 context window 请求时进入 compaction，再继续 sampling，而不是直接丢弃整个 turn。
- [`codex-rs/core/src/session/mcp.rs`](../codex-rs/core/src/session/mcp.rs) 为每个 step 投影 MCP runtime；当有效 server/connector 或环境可用性变化时，串行刷新并发布新的 `McpRuntimeSnapshot`。

不变量：MCP 工具不是静态全局列表；它取决于当前 turn 配置、可用环境、认证与 runtime snapshot。

## 阶段 2 验收

- [x] 从 CLI 到模型采样和工具回填的链路已定位。
- [x] 已识别用户输入、history、step context、tool router 和 runtime 的职责边界。
- [x] 已解释 approval/sandbox、compaction、MCP runtime 的介入点。
- [x] 已将结论与阶段 1 的 JSONL、权限和 TUI 行为实验交叉检查。

## 未深入的内容

- 每个 slash command 的 UI 渲染与 state machine。
- 各工具 handler 的平台差异和所有 retry 分支。
- app-server、SDK、remote-control 和 cloud task 的协议细节。

这些属于阶段 3 或按真实问题回读的范围，而不是阶段 2 的阻塞项。
