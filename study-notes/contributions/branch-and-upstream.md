# 分支与上游同步

## 同步结果

2026-07-10 获取 `upstream/main` 后：

- `upstream/main`：`6138909d6ec58b2fbe635ef973e02caecad5a5aa`
- 同步前 `origin/main`：`cca16a10878202cb2f6e9666b6b4330329ea7e65`
- 差异：fork 落后 119、没有独有提交，即 `0 119`
- 先用 `merge-base --is-ancestor` 确认纯 fast-forward，再推送 `upstream/main:main`
- 同步后 `origin/main` 与 `upstream/main` 同为 `6138909d6e`

没有 checkout/merge 学习分支，没有 force push、rebase、reset 或 checkout 丢弃。

## 分支职责

```text
upstream/main 6138909d6e
      ├── origin/main 6138909d6e
      └── agent/plugin-creator-hooks-validation
            └── 53a917a3ac Accept plugin hooks in bundled validator

00-study/codex-agent-learning
      └── 阶段笔记、实验、issue 回链（不承载上游修复）
```

贡献分支在独立 worktree `/root/codex-stage4` 创建；学习分支仍在 `/root/codex`。两个工作区通过路径和分支职责隔离，避免为写学习笔记污染代码补丁。

## 远程证据

- `origin/main`：`6138909d6e`
- `origin/agent/plugin-creator-hooks-validation`：`53a917a3ac`
- `origin/00-study/codex-agent-learning`：阶段三提交后为 `4c36cd159f`

## 维护规则

1. 学习开始前 fetch upstream，不自动 merge 当前学习分支。
2. 每个贡献从最新 upstream main 创建单独 branch/worktree。
3. 只有确认 fork main 是 upstream main 的祖先时才 fast-forward。
4. 上游变化后重新复现和测试；不通过 force 处理冲突。
