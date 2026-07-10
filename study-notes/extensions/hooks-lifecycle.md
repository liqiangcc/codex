# Hooks 生命周期

## 生命周期链路

```text
config.toml or .codex/hooks.json
  -> config layer identifies source and project root
  -> discover_handlers parses event groups and matchers
  -> enabled/trusted state selects handlers
  -> runtime serializes event-specific JSON to stdin
  -> command runs outside the sandbox with timeout
  -> stdout parser maps JSON/exit/error to HookCompleted outcome
```

## 源码证据

- `codex-rs/hooks/src/engine/discovery.rs::discover_handlers` 合并 managed、config、JSON 和 plugin 来源，生成 handler、列表项和 warning。
- `codex-rs/hooks/src/config_rules.rs::hook_states_from_stack` 只允许 user/session flag 层改变 enable/trust state，项目或插件不能替用户信任自己。
- `codex-rs/hooks/src/engine/command_runner.rs::run_command` 通过 stdin 输入、stdout/stderr 捕获、timeout 和 `kill_on_drop` 运行 command。
- `codex-rs/hooks/src/events/stop.rs::run` 选择 Stop handler、序列化 `StopCommandInput`、执行并聚合结果。
- `codex-rs/hooks/src/engine/output_parser.rs::parse_stop` 解析 bounded JSON 输出；空或非法输出按失败语义处理。
- `codex-rs/core/src/hook_runtime.rs::run_stop_hooks` 是 agent turn 停止点的接入位置。

## 隔离实验

`experiments/20260710-hooks/` 包含：

- `hooks.json`：一个 5 秒 timeout 的 Stop command handler。
- `stop_hook.py`：只读 stdin、验证事件/必填字段、输出 `continue:true`；无文件和网络 I/O。
- `valid-input.json` 与 `invalid-input.json`：固定、redacted fixtures。

行为结果：

- 有效输入退出 0，stdout 为 `{"continue": true, "systemMessage": "read-only study Stop hook observed"}`。
- 错误事件退出 2，stderr 为 `expected hook_event_name=Stop`。
- `hooks.json` 通过 JSON parser。
- 对应 Rust runtime 通过 `just test -p codex-hooks` 验证；首次运行需要安装 Rust 1.95、`just` 和 `cargo-nextest`。

未使用 `--dangerously-bypass-hook-trust` 做 live 执行。原因是 command hook 在 sandbox 外运行，而本阶段目标可以用直接 handler fixtures 与 runtime crate tests 完成，不需要扩大当前 Codex 的 trust。

## 失败与安全语义

- 未信任 command hook 应显示为需要 review，不应静默执行。
- 非零退出、spawn error、stdin write error 或 timeout 形成 failed hook outcome。
- `continue:false`、`decision:block` 和 `reason` 由事件专用 parser 解释，不能用任意 stdout 模糊控制。
- 删除项目 hook 文件即可卸载；实验没有写用户级 trust state。
- Hook 没有放进观察器插件默认路径，避免所有 Stop 事件无条件增加 sandbox 外执行面。

## 权限、数据、上下文与清理边界

- Command Hook 在 Codex sandbox 外运行，因此 trust/enablement 与 agent sandbox 是两个独立权限边界；项目和插件不能替用户写入信任。
- 本实验只接收一个 event JSON，输出一个固定、小型 JSON；不读 transcript 文件、不访问网络、不接收或保存凭据。
- 只有事件专用 parser 明确接受的 `additionalContext`/continuation 才能进入模型上下文；任意 stdout 不能被默认当作无限上下文。
- Runtime 对大 Hook 输出使用 `codex-rs/hooks/src/output_spill.rs` 的 spill 路径，实验本身进一步把输出限制为单行结果。
- Handler 由 timeout 和 `kill_on_drop` 约束；直接 fixture 不留下子进程，项目 Hook 文件删除后即无可发现 handler。
