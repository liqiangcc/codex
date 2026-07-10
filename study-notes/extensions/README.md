# 阶段 3：扩展与运行机制证据索引

阶段三以“观察 Codex TUI 并记录学习证据”为同一条主线，分别验证扩展选择、Skill、MCP、Hook、Plugin 和组合边界。

| 主题 | 结论 | 行为证据 |
| --- | --- | --- |
| [扩展层级决策](extension-decision.md) | 指令复用用 Skill，新增受控能力用 MCP，事件触发用 Hook，组合分发用 Plugin | 五类需求决策表 |
| [Skills 生命周期](skills-lifecycle.md) | 新线程加载，匹配后注入完整 `SKILL.md`，UI 元数据与指令分离 | 新线程 `/skills` 可见 `Observe Codex Study` |
| [MCP 工具与权限](mcp-tools-and-permissions.md) | server 只暴露固定 read-only Codex 子进程，不接受任意 executable | 启动、`/skills`、未知 session 失败、stop 清理 |
| [Hooks 生命周期](hooks-lifecycle.md) | command hook 在 sandbox 外运行，必须单独审查 trust | Stop handler 成功/失败输入和 `codex-hooks` 测试 |
| [Plugin 分发](plugin-distribution.md) | manifest 聚合 Skill/MCP，cachebuster 形成不可变安装版本 | 校验、升级、重装、缓存内容和启用状态 |
| [学习自动化闭环](learning-automation-loop.md) | Plugin 负责分发，Skill 负责编排，MCP 负责观察；Hook 不进入默认链路 | 综合启动、发现、调用、失败、清理、升级复盘 |

共同安全边界：不执行 `/usage`、登出、账户切换、额度重置、认证变更或破坏性 Git 命令；不保存 token、email、session ID 和原始账号输出。
