## 目标

保持 upstream main、fork main、学习分支和贡献分支职责分离。

## 完成证据

- upstream main 获取到 `6138909d6e`。
- 确认 fork main 仅落后 119 且可纯 fast-forward 后完成同步。
- 从 upstream main 创建独立 worktree/branch。
- 学习分支没有承载代码补丁。

详细记录：[分支与上游同步](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/contributions/branch-and-upstream.md)

## 结论

同步与贡献均未使用 force、reset、rebase 或丢弃式 checkout。
