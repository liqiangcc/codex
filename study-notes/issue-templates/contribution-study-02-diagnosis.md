> 状态：Done
>
> 当前结论：Plugin Hook validator 漂移已被归类为真实 developer-tool bug。
>
> 下一步：无；若 upstream 先修复则关闭本地候选补丁。

## 目标

为一个真实 upstream 问题完成分类和最小复现。

## 完成证据

- 核心 loader 和 plugin-creator 示例支持 `hooks`，bundled validator 却拒绝。
- 修复前回归测试 2/2 失败于 `hooks` unknown field。
- 已排除账号、配置和环境问题。

详细记录：[问题诊断与复现](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/contributions/diagnosis-and-reproduction.md)

## 结论

这是 bundled developer tool 与 core contract 漂移造成的真实 bug。
