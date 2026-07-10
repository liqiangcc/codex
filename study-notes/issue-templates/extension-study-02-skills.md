> 状态：Done
>
> 当前结论：Skills 生命周期已完成源码与新线程行为验证。
>
> 下一步：无；仅在 Skill discovery/injection 行为变化时重新验证。

## 目标

理解 Skill 的发现、匹配、指令注入、配置禁用和插件来源。

## 完成证据

- 已创建 instruction-only `$observe-codex-study` Skill。
- Skill 与 Plugin validator 均通过。
- 重装后在全新 Codex TUI 的 `/skills` 列表中观察到 `Observe Codex Study`。
- 已记录一级菜单误判的预期失败路径与新线程加载边界。

详细记录：[Skills 生命周期](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/extensions/skills-lifecycle.md)

## 结论

Skill 适合复用编排与安全指令；完整正文只在触发后进入上下文，更新后需要新 cache 和新线程。
