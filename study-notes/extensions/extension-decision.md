# 扩展层级决策

## 决策顺序

先问“是否需要新增能力”，再问“是否需要事件触发”，最后才问“是否需要打包分发”。这样可以避免把一段可复用指令直接做成长期运行的 server。

1. 单次、上下文已完整：直接 prompt。
2. 项目长期约束：`AGENTS.md` 或同类 guidance。
3. 可复用工作流、无需新增系统能力：Skill。
4. 需要模型调用受控外部能力：MCP。
5. 必须在生命周期事件自动运行：Hook。
6. 多种组件需要统一安装、版本和开关：Plugin。

## 五类重复流程决策

| 需求 | 选择 | 不选择更重方案的原因 |
| --- | --- | --- |
| 每次修改 Rust 后遵循本仓库测试命令 | `AGENTS.md` | 属于项目约束，不需要每次显式触发 Skill |
| 把一次 TUI 观察整理成固定证据格式 | Skill | 只是可复用编排和安全规则，不需要常驻进程 |
| 从 Codex 内启动并观察另一个只读 Codex TUI | MCP | 需要 PTY 生命周期和结构化工具调用，prompt 无法提供能力 |
| 每次 Stop 时做只读合规检查 | Hook | 需要事件触发；普通 Skill 不能保证每次 Stop 自动运行 |
| 同时分发观察 Skill 和 MCP server | Plugin | 需要统一 manifest、安装版本、启用和卸载边界 |

本阶段的最终组合是 `Plugin(Skill + MCP)`。Stop Hook 只保留为隔离实验，没有放进默认插件：学习记录不是每次 turn 必须执行的硬性事件，默认启用 Hook 会增加 sandbox 外执行面。

## 源码证据

- `codex-rs/core-skills/src/service.rs` 的 `SkillsService::snapshot_for_config` 和 `skill_roots_for_config` 负责按配置形成可用 Skill 快照与 root。
- `codex-rs/core/src/session/mcp.rs` 的 `runtime_mcp_config`、`mcp_runtime_for_step` 和 `refresh_mcp_servers_now` 把 MCP 配置变成 step 运行时能力。
- `codex-rs/hooks/src/engine/discovery.rs` 的 `discover_handlers` 合并 config、managed 和 plugin hook 来源，再计算 trust/enablement。
- `codex-rs/core-plugins/src/loader.rs` 的 `load_plugins_from_layer_stack` 与 `load_plugin_skills` 聚合插件组件。
- `codex-rs/plugin/src/load_outcome.rs` 的 `effective_skill_roots`、`effective_mcp_servers` 和 `effective_plugin_hook_sources` 只返回 active plugin 的贡献。

## 成功与失败路径

- 成功：同一插件中的 Skill 在新线程 `/skills` 列表可见，MCP server 能启动只读子 TUI。
- 失败：如果只写 Skill 指令而没有 MCP，Codex 没有 PTY 能力；如果 MCP 接受任意 command，则权限边界过大；如果把记录流程做成默认 Stop Hook，则每轮都产生不必要的 sandbox 外执行。

## 边界

- Prompt/guidance/Skill 会占模型上下文；Skill 只在触发时加载完整正文。
- MCP 工具参数和返回必须有硬上限，server 权限不能高于任务所需。
- Command Hook 在 sandbox 外运行，trust 是独立安全边界。
- Plugin 是分发容器，不应被误当成新的权限模型。
