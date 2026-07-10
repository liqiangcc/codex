## 目标

为 prompt、`AGENTS.md`、Skill、MCP、Hook 和 Plugin 建立可复用的最轻层级决策法。

## 完成证据

- 已为五类重复流程给出选择与排除理由。
- 已说明上下文、权限、事件触发和分发边界。
- 最终选择 `Plugin(Skill + MCP)`，不把可选学习记录做成默认 Hook。

详细记录：[扩展层级决策](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/extensions/extension-decision.md)

## 结论

只有在缺少能力时才引入 MCP，只有在不可遗漏的生命周期事件中才引入 Hook，Plugin 只承担组合分发和版本边界。
