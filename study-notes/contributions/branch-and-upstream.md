# 分支与上游同步

## 同步结果

2026-07-10 创建贡献分支时获取 `upstream/main` 后：

- `upstream/main`：`6138909d6ec58b2fbe635ef973e02caecad5a5aa`
- 同步前 `origin/main`：`cca16a10878202cb2f6e9666b6b4330329ea7e65`
- 差异：fork 落后 119、没有独有提交，即 `0 119`
- 先用 `merge-base --is-ancestor` 确认纯 fast-forward，再推送 `upstream/main:main`
- 首次同步后 `origin/main` 与 `upstream/main` 同为 `6138909d6e`

最终验收期间 upstream 又前进；最后一次 fetch 得到 `2b9c05046038c038ec6bddb9db7d11394995372d`：

- `origin/main` 再次纯 fast-forward 到 `2b9c050460`。
- 学习分支用普通 merge 合入最新 main，最终 merge 提交为 `df2d6063ec`；包含最终审计提交后相对 upstream 为 `20 ahead / 0 behind`。
- 贡献分支也用普通 merge 刷新，当前 head 为 `d7d109c3f6`，相对 upstream 为 `3 ahead / 0 behind`。
- 刷新前确认最新 upstream 仍存在 validator bug；最后一次刷新后重新运行 Python 7/7 和 `codex-skills` 1/1。

首次同步 fork main 时没有 checkout/merge 学习分支；最终刷新只做了普通 merge。全程没有 force push、rebase、reset 或丢弃式 checkout。

## 分支职责

```text
upstream/main 2b9c050460
      ├── origin/main 2b9c050460
      └── agent/plugin-creator-hooks-validation
            ├── 53a917a3ac Accept plugin hooks in bundled validator
            └── d7d109c3f6 final refresh merge head

00-study/codex-agent-learning
      └── df2d6063ec 阶段笔记、实验、issue 回链与最新 main merge
```

贡献分支在独立 worktree `/root/codex-stage4` 创建；学习分支仍在 `/root/codex`。两个工作区通过路径和分支职责隔离，避免为写学习笔记污染代码补丁。

## 远程证据

- `origin/main`：`2b9c050460`
- `origin/agent/plugin-creator-hooks-validation`：`d7d109c3f6`
- `origin/00-study/codex-agent-learning`：最终学习审计提交通过正常 push 发布；其父 merge 为 `df2d6063ec`，且不落后 upstream

## 维护规则

1. 学习开始前 fetch upstream，不自动 merge 当前学习分支。
2. 每个贡献从最新 upstream main 创建单独 branch/worktree。
3. 只有确认 fork main 是 upstream main 的祖先时才 fast-forward。
4. 上游变化后重新复现和测试；不通过 force 处理冲突。
