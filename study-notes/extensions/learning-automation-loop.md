# 学习自动化闭环

## 需求

重复流程是：在 Codex 内启动一个新的只读 Codex TUI，观察安全 slash command，把实际行为与源码交叉验证，并留下小而可复现的笔记。

## 最终架构

```text
personal plugin
├── observe-codex-study Skill
│   ├── 选择安全问题与命令
│   ├── 编排 start/wait/send/screen/stop
│   └── 约束 redaction、证据格式与失败清理
└── codex-tui-observer MCP
    ├── 固定 read-only Codex 子进程
    ├── bounded PTY 输入/等待/屏幕
    └── redaction 与进程清理
```

Hook 不在默认组合内。它适合不可遗漏的事件策略，而“记录一次学习观察”应由用户显式触发；强制 Stop Hook 会把可选流程变成每 turn 的 sandbox 外 command。

## 端到端验收

| 路径 | 证据 |
| --- | --- |
| 启动 | MCP 返回 read-only sandbox 和有效 session |
| 发现 | 新线程 `/skills` 列出 `Observe Codex Study` |
| 调用 | `/skills` 由固定 text + Enter 驱动，进入列表后命中显示名 |
| 权限 | executable、CLI args、sandbox、approval policy 均不可由调用者修改 |
| 错误 | 未知 session 返回确定性 ValueError；一级菜单误判被识别为状态问题 |
| 清理 | finally 中显式 `stop` 返回 true |
| 升级 | cachebuster 更新、personal marketplace 重装、不可变 cache 内容核对 |
| 回滚 | 禁用/卸载 plugin 移除 Skill 与 MCP；Hook 实验只需删除项目配置 |

## 源码交叉验证

- Skill：`core-skills::SkillsService` 和 plugin `load_plugin_skills`。
- MCP：session `mcp_runtime_for_step` 与 plugin `effective_mcp_servers`。
- Hook：`discover_handlers`、`run_command` 和 Stop event parser。
- Plugin：manifest loader、`PluginStore` 和 `PluginLoadOutcome::effective_*`。

## 数据和上下文预算

- 输入最多 4096 bytes，屏幕/转录最多 256000 bytes，wait 最多 30 秒。
- Skill 要求总结行为，不粘贴完整 transcript。
- email、常见 token 和 credential 值在 server 返回前 redaction。
- Skill 正文只在触发时注入；Plugin capability summary 只保留小型安全描述。
- 笔记不得含 session ID、账号标识、认证状态细节或 secret。

## 结论

这个组合是足以自动化该学习流程的最轻方案：Skill 提供可维护流程，MCP 提供缺失的 PTY 能力，Plugin 统一分发和版本。新增 Hook、App 或 SDK 都不会提高当前目标的完成度，反而扩大权限面、上下文或维护成本。
