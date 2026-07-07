# 功能：调试和诊断

## 1. 一句话说明

当 Codex 行为异常时，用 doctor、status、debug-config、usage、logs、JSONL、Git 状态和最小复现定位问题来源。

## 2. 背景和动机

Codex 失败原因很多：认证失效、模型不可用、配置冲突、sandbox 限制、network 关闭、MCP 初始化失败、hook 未 trust、Git 状态混乱、上下文漂移、prompt 不清、上游版本变化。没有诊断流程，就只能猜。

第一阶段要把诊断当成固定能力学习，而不是出问题才临时搜索。一个可复现的问题报告应该包含：现象、环境、版本、命令、配置摘要、权限状态、Git 状态、日志摘要、最小复现、期望行为、实际行为、脱敏说明。

## 3. 用户目标

- 能快速收集 Codex 本地运行证据。
- 能区分环境问题、配置问题、权限问题、上下文问题和模型输出质量问题。
- 能用 `/status`、`/debug-config`、`codex doctor`、`codex debug models` 进行初步定位。
- 能处理长时间运行命令、后台终端和 stuck 状态。
- 能把诊断信息整理成 issue，而不是粘贴一大堆日志。

## 4. 适用场景

- Codex 启动失败或登录失败。
- 模型列表、默认模型、rate limit 或 usage 异常。
- 命令被 sandbox 阻止或审批异常。
- 配置没生效。
- MCP、hook、plugin、skill 不可用。
- `codex exec` 输出不符合预期或 JSONL 解析失败。
- TUI 卡住、后台命令不结束。
- CLI、app、IDE 行为不一致。

不适合把所有质量问题都当 bug。若 prompt 模糊、上下文不足或任务过大，应该先修任务表达。

## 5. 功能边界

- 诊断命令提供证据，不保证自动修复。
- `codex doctor` 面向安装、配置、认证、runtime、Git、terminal、app-server、thread inventory 等本地问题。
- `/status` 面向当前 session 状态。
- `/debug-config` 面向配置层、requirements、实验网络约束等。
- `/usage` 面向账号 token 使用和 rate limit 信息。
- `/ps`、`/stop` 面向后台终端。
- `/feedback` 用于向维护者反馈问题，分享前必须脱敏。

## 6. 使用约束

- 诊断输出可能包含路径、账号、workspace、配置、日志位置，分享前必须审查。
- 不要在 issue 中粘贴 access token、API key、auth cache、完整 secret env。
- 先做最小复现，再提交大段日志。
- 如果任务失败来自权限或配置，先记录实际状态，不要直接放宽到 full access。
- 对当前日期、版本、模型、上游文档变化保持敏感，必要时重新查 manual。

## 7. 用户入口

- `codex doctor`：本地诊断。
- `codex doctor --json`：如果当前版本支持，用于结构化诊断观察。
- `codex --version`：版本。
- `codex debug models`、`codex debug models --bundled`：模型 catalog。
- `/status`：session 配置、token、权限、workspace。
- `/usage`：账号用量。
- `/debug-config`：配置层诊断。
- `/ps`：查看后台终端和最近输出。
- `/stop`：停止后台终端。
- `/feedback`：提交反馈和日志。
- `codex exec --json`：观察非交互事件流。
- `git status --short --branch`、`git diff --stat`：确认仓库状态。

## 8. 典型工作流

启动问题：

1. `codex --version`。
2. `codex doctor`。
3. 检查认证方式和 `CODEX_HOME`。
4. 记录错误消息和环境。
5. 尝试最小启动 prompt。

配置问题：

1. `/status` 看实际状态。
2. `/debug-config` 看 layer。
3. 检查 CLI flags、project config、profile、user config。
4. 使用 `--strict-config` 检查未知 key。
5. 用一次性 `-c` 验证假设。

权限问题：

1. `/status` 看 sandbox、approval、writable roots。
2. 记录被阻止的命令或工具。
3. 判断是否应该审批、改配置、加 rule，还是保持拒绝。
4. 做最小权限复现。

Stuck 问题：

1. 判断是否等待 approval。
2. `/ps` 查看后台命令。
3. 必要时 `/stop`。
4. 新开更小 prompt 复现。
5. 记录 stuck 前最后一个操作。

## 9. 最佳实践

- 诊断从“事实收集”开始，不从“猜修复”开始。
- 每个问题先写一行：期望行为、实际行为、复现命令。
- 对 CLI/app/IDE 差异，分别确认版本。
- 对配置问题，用 `/debug-config` 代替人工推测。
- 对权限问题，记录当前模式和被请求动作。
- 对模型或输出质量问题，保留最小 prompt 和必要上下文。
- 对日志做摘要：关键错误、时间点、命令，不直接贴完整敏感文件。

## 10. 常见错误

- 只说“Codex 不工作”，没有版本和命令。
- 把权限失败当成模型问题。
- 把 prompt 模糊当成 bug。
- 分享 doctor/logs 前不脱敏。
- 遇到 stuck 直接杀进程，丢失可复现信息。
- 配置改来改去，没有记录每次变化。
- 忽略 Git dirty state，误以为 Codex 改了不该改的文件。

## 11. 风险和限制

- 诊断输出可能包含隐私或敏感路径。
- 某些问题来自远端服务、账号权限或 rollout，本地无法完全修复。
- CLI、app、IDE 的版本可能不一致。
- `doctor` 输出项随版本变化，学习笔记要定期复核。
- 最小复现如果过度简化，可能丢掉触发条件。

## 12. 验证方式

最小诊断包：

```bash
codex --version
codex doctor
git status --short --branch
```

TUI 内：

```text
/status
/debug-config
/usage
```

非交互：

```bash
codex exec --json --sandbox read-only "Summarize this repository" | jq -r '.type'
```

修复验证：

- 重跑最小复现。
- 对比修复前后诊断字段。
- 如果涉及文件，检查 `git diff`。
- 如果涉及配置，重开 session 确认生效。

## 13. 最小练习

创建一条“诊断模板”记录到 issue #9：

- 现象：
- 期望：
- 复现命令：
- `codex --version`：
- `codex doctor` 关键项：
- `/status` 关键项：
- `/debug-config` 关键项：
- Git 状态：
- 已脱敏字段：
- 下一步：

## 14. 进阶练习

- 用 `--strict-config` 制造一个配置错误，记录 Codex 如何报告。
- 用 read-only 模式请求写文件，记录权限错误路径。
- 用 `codex exec --json` 捕获一次失败任务，分析 `turn.failed` 或 `error`。
- 触发一个长命令后使用 `/ps` 和 `/stop` 观察行为。
- 对比 CLI `codex --version` 和 app bundled Codex version。

## 15. 实验设计

实验目标：建立 Codex 问题分类表。

问题类型：

- 安装/版本。
- 认证。
- 配置。
- 权限。
- 网络。
- 工具扩展。
- Git 状态。
- 上下文/prompt。
- 远端服务。

每类记录：

- 典型现象。
- 首选诊断命令。
- 必要证据。
- 常见修复。
- 何时开 issue。

产出：

- 在 `checks.md` 或本文件沉淀最小诊断流程。
- 在 issue #9 持续记录真实案例。

## 16. 相关文档

- OpenAI Codex manual：Troubleshooting。
- OpenAI Codex manual：CLI command reference，`codex doctor`、`codex debug models`。
- OpenAI Codex manual：Slash commands in Codex CLI。
- OpenAI Codex manual：Authentication and sessions。
- OpenAI Codex manual：Config basics。
- 本仓库：`docs/getting-started.md`。
- 本仓库：`docs/config.md`。

## 17. 源码入口

- `codex-rs/cli/src/doctor.rs` 和 `codex-rs/cli/src/doctor/`：doctor 诊断。
- `codex-rs/tui/src/debug_config.rs`：TUI debug config。
- `codex-rs/feedback/src/feedback_diagnostics.rs`：feedback 诊断打包。
- `codex-rs/models-manager/src/`：模型 catalog 管理。
- `codex-rs/config/src/diagnostics.rs`：配置诊断。
- `codex-rs/core/src/unified_exec/`：后台命令、PTY 和进程状态。
- `codex-rs/core/src/stream_events_utils.rs`：事件流辅助逻辑。

## 18. 待解决问题

- `codex doctor --json` 当前版本是否稳定，字段如何命名？
- `/usage` 的数据来源和 rate limit 显示逻辑在哪里？
- `/ps` 和 `/stop` 管理的是哪些后台进程，跨 turn 是否保留？
- feedback 上传包含哪些日志，哪些需要用户确认？
- 哪些诊断信息适合写入公开 GitHub issue？

## 19. 当前结论

调试和诊断要先事实后修复。阶段 1 应形成固定诊断包：版本、doctor、status、debug-config、Git 状态、最小复现、脱敏说明。这样遇到问题时才能定位到环境、配置、权限、上下文或真实 bug。
