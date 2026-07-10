> 状态：Done
>
> 当前结论：`Plugin(Skill + MCP)` 学习自动化闭环已完成端到端验收。
>
> 下一步：无；除非需求需要事件强制执行，否则不增加 Hook/App/SDK。

## 目标

完成“观察 TUI 并记录结果”的启动、发现、调用、权限、错误、清理、升级和回滚闭环。

## 完成证据

- Plugin 统一分发 Skill 与 MCP。
- Skill 编排安全观察和结构化证据。
- MCP 提供固定只读 PTY 生命周期。
- Hook 经过隔离实验后被排除在默认组合外。
- 所有输出有界、redacted，且不记录账号、token 或 session ID。

详细记录：[学习自动化闭环](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/extensions/learning-automation-loop.md)

## 结论

`Plugin(Skill + MCP)` 是当前目标的最轻完整方案；App、SDK 和默认 Hook 都属于过度设计。
