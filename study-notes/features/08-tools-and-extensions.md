# 功能：工具和扩展

## 1. 一句话说明

通过 MCP、skills、hooks、plugins、apps/connectors 和 subagents 扩展 Codex 的工具、工作流、外部系统和协作能力。

## 2. 背景和动机

Codex 内置能力解决通用 coding 工作，但真实工作还需要私有文档、issue 系统、设计工具、浏览器、监控日志、团队流程、安全检查和可复用任务步骤。扩展机制让 Codex 不只依赖 prompt，而能接入工具、读取资源、执行标准流程，并把能力打包分发。

第一阶段不需要深入开发插件或 MCP server，但必须知道每类扩展解决什么问题、什么时候该用、风险在哪里。否则容易把所有问题都写成 prompt，或者为了简单需求引入过重扩展。

## 3. 用户目标

- 能区分 MCP、skill、hook、plugin、app connector、subagent 的职责。
- 能判断某个需求应该用普通 prompt、`AGENTS.md`、skill、MCP 还是 plugin。
- 能用 `/skills`、`/mcp`、`/hooks`、`/plugins`、`/apps` 检查当前可用能力。
- 能理解扩展带来的权限、认证和数据共享风险。
- 能为学习体系设计“先 prompt 验证，再 skill 固化，再 plugin 分发”的演进路径。

## 4. 适用场景

- 接入 GitHub、Linear、Figma、Sentry、内部 docs、浏览器等外部工具。
- 把重复学习流程固化成 skill。
- 在工具调用前后运行安全或质量检查。
- 安装团队共享的能力包。
- 把复杂任务分给不同角色的 subagents。
- 把 app connector 或 MCP 工具和 skill 组合成工作流。

不适合：

- 一次性的简单解释或小改动。
- 还没验证过价值的流程。
- 权限边界不清的外部系统。
- 只为了“看起来高级”而引入插件。

## 5. 功能边界

- Skill 是可复用工作流和专业知识，核心是 instructions、references、scripts。
- MCP 是外部工具和资源协议，提供 tools、resources、prompts。
- Hook 是生命周期扩展，在特定事件前后运行脚本或检查。
- Plugin 是安装和分发单位，可以打包 skills、apps、MCP servers、hooks、资产。
- App connector 是授权外部服务的入口，适合私有应用数据和动作。
- Subagent 是多 agent 协作方式，适合并行或专业角色任务，但会增加 token 和管理成本。

## 6. 使用约束

- 任何外部工具都必须明确认证方式、数据范围、写入权限和审批模式。
- Skill description 要写清触发边界，否则隐式调用不稳定。
- Hooks 需要 trust；非 managed hook 变化后要重新审查。
- Plugin 安装后不代表外部 app 已授权，可能还需要连接或登录。
- MCP destructive 或 side-effecting tool calls 仍可能触发 approval。
- Subagents 只在明确要求时使用，不应作为所有任务默认策略。

## 7. 用户入口

- `/skills` 或 `$skill-name`：选择或显式调用 skill。
- `.agents/skills/`、`$HOME/.agents/skills`、`/etc/codex/skills`：skill 存放位置。
- `codex mcp`：管理 MCP servers。
- `/mcp`：查看当前 MCP tools。
- `[mcp_servers.*]`：在 config 中配置 MCP。
- `/hooks`：查看、trust、禁用 hooks。
- `hooks.json` 或 `[hooks]`：配置 hooks。
- `/plugins`、`codex plugin`、`codex plugin marketplace`：浏览、安装、管理 plugins。
- `/apps`：浏览 app connectors。
- `/agent`、subagent 配置 `[agents]`：切换或使用 subagents。

## 8. 典型工作流

从 prompt 到 skill：

1. 先用普通 prompt 手动跑流程。
2. 连续使用 3 次以上仍有价值。
3. 抽出输入、步骤、输出、验证、边界。
4. 写成 skill。
5. 用 `/skills` 或 `$skill` 测试触发。
6. 若要分享，打包成 plugin。

接入外部系统：

1. 判断是否需要 live data 或外部 action。
2. 优先使用已有 plugin 或 MCP server。
3. 配置认证和 approval。
4. 用只读工具做最小调用。
5. 再允许写操作或 side effects。
6. 记录数据边界和失败路径。

Hooks：

1. 先明确要拦截哪个生命周期事件。
2. 写最小命令 hook。
3. 检查 `/hooks`，审查并 trust。
4. 触发一次任务验证。
5. 记录 hook 输出、timeout、失败行为。

## 9. 最佳实践

- 普通 prompt 能解决的一次性问题，不要升级为扩展。
- 重复流程优先 skill；需要外部工具时 skill + MCP；要分享时 plugin。
- Hooks 适合 enforce 和 observe，不适合承载大量业务逻辑。
- MCP server instructions 前 512 字符要自包含，便于 Codex 决定是否使用。
- 对每个扩展写清“能读什么、能写什么、什么时候会请求 approval”。
- 对新 plugin 先看 manifest、bundled skills、MCP servers、hooks，再安装使用。
- Subagents 用于拆分有清晰边界的并行任务，不用于逃避任务拆解。

## 10. 常见错误

- 把 skill 当成 prompt 模板，只写泛泛建议，没有输入输出和步骤。
- 把 MCP 当成知识库，忽略它可能执行 action。
- 安装 plugin 后不看它带了哪些 MCP servers 或 hooks。
- Hook 命令使用相对路径，Codex 从子目录启动时找不到。
- 为简单任务写插件，增加维护成本。
- 让 subagents 并行处理高度耦合任务，最后合并成本更高。
- 不记录外部系统授权范围，后续无法排查为什么工具不可用。

## 11. 风险和限制

- 扩展扩大了 Codex 能接触的数据和动作范围。
- MCP、apps、plugins 的授权失败可能表现为工具不可用，而不是 prompt 问题。
- Hooks 运行外部命令，可能引入安全和性能风险。
- Skill 太多会占用初始 skill 列表预算，描述会被缩短或部分省略。
- Plugin 市场源和本地源需要信任审查。
- Subagents 增加 token 消耗，也增加状态管理复杂度。

## 12. 验证方式

能力发现：

```text
/skills
/mcp
/hooks
/plugins
/apps
/status
```

MCP 验证：

```bash
codex mcp --help
```

Skill 验证：

- 显式调用 `$skill-name`。
- 观察 Codex 是否读取 `SKILL.md`。
- 检查输出是否遵守 skill 的步骤和边界。

Hook 验证：

- `/hooks` 中确认 source、event、matcher、trust 状态。
- 触发对应 event。
- 检查 hook 输出和失败行为。

## 13. 最小练习

完成一次扩展盘点：

1. 在 TUI 中查看 `/skills`。
2. 查看 `/mcp`。
3. 查看 `/hooks`。
4. 查看 `/plugins`。
5. 在 issue #8 记录：当前可用扩展、每类扩展的用途、一个最适合学习体系的扩展候选。

## 14. 进阶练习

- 为“更新 feature docs + issue 摘要”设计一个 instruction-only skill 草稿。
- 配置一个只读 MCP docs server，完成一次文档查询。
- 写一个 Stop hook，在任务结束时检查 `git diff --check`，先在临时项目验证。
- 安装一个可信 plugin，记录它带来的 skills、apps、MCP servers。
- 设计一个 subagent 适用场景，例如“一个 agent 做源码路径搜索，主 agent 负责总结”。

## 15. 实验设计

实验目标：确定学习体系是否需要扩展化。

候选流程：

- 每次学习后检查 feature doc 必备章节。
- issue 摘要同步。
- 上游 manual 更新后扫描受影响功能。
- 源码路径阅读导航。

评估维度：

- 是否重复。
- 是否有清晰输入输出。
- 是否需要脚本。
- 是否需要 GitHub/MCP 外部工具。
- 是否适合个人使用还是团队分享。

产出：

- 先写普通流程。
- 再判断是否升级为 skill。
- 最后才考虑 plugin。

## 16. 相关文档

- OpenAI Codex manual：Customization。
- OpenAI Codex manual：Agent Skills。
- OpenAI Codex manual：Model Context Protocol。
- OpenAI Codex manual：Hooks。
- OpenAI Codex manual：Plugins。
- OpenAI Codex manual：Subagents。
- 本仓库：`docs/skills.md`。

## 17. 源码入口

- `codex-rs/core-skills/src/`：skills 加载、注入、渲染。
- `codex-rs/skills/src/`：skills 相关 crate。
- `codex-rs/codex-mcp/src/`：MCP 连接管理。
- `codex-rs/core/src/mcp.rs`、`codex-rs/core/src/mcp_tool_call.rs`：core MCP 工具调用。
- `codex-rs/hooks/src/`：hooks 事件、发现、执行、输出解析。
- `codex-rs/core-plugins/src/`：plugin 安装、市场、加载。
- `codex-rs/plugin/src/`：plugin manifest 和基础类型。
- `codex-rs/core/src/agent/`：subagent 角色和注册。

## 18. 待解决问题

- Skill 隐式触发的 scoring 或匹配逻辑在哪里实现？
- MCP tool approval mode 如何和 session approval policy 合并？
- Plugin-bundled hooks 与普通 hooks 的 trust 流程是否完全一致？
- Apps/connectors 和 MCP 在 core tool exposure 中如何区分？
- 学习体系是否应该先做 skill，还是保持文档和 issue 流程即可？

## 19. 当前结论

工具和扩展应按成熟度递进：一次性任务用 prompt，长期仓库规则用 `AGENTS.md`，重复流程用 skill，需要外部系统用 MCP/app connector，要分发再用 plugin。阶段 1 重点是分辨边界和风险，不急于开发复杂扩展。

2026-07-10 最小练习结果：`/skills`、`/hooks`、`/plugins`、`/apps` 均打开对应管理入口；`/mcp` 显示当前会话的 `codex_apps` 工具服务。CLI `codex mcp list` 显示本地 `codex-tui-observer` MCP 已启用，`codex plugin list` 显示它由 personal marketplace 安装。结论是 skill 负责任务流程，MCP 暴露工具，plugin 打包分发这些能力；学习观察器应保持只读，避免把观测能力扩展为默认写权限。
