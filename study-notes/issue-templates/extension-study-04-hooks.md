> 状态：Done
>
> 当前结论：Hook discovery、trust、事件和失败语义已完成隔离验证。
>
> 下一步：无；不把实验 Hook 加入默认运行链路。

## 目标

理解 Hook 的来源、trust、matcher、事件输入、运行和失败语义。

## 完成证据

- 已实现隔离的只读 Stop handler 与有效/无效 fixtures。
- 成功路径退出 0，错误事件路径退出 2。
- `codex-hooks` 120/120 测试通过。
- 未使用 hook trust bypass，未写用户级 trust state。

详细记录：[Hooks 生命周期](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/extensions/hooks-lifecycle.md)

## 结论

Command Hook 在 sandbox 外运行，项目或插件不能替用户信任自己；本学习流程不需要默认 Hook。
