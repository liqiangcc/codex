# 功能：Git 和协作

## 1. 一句话说明

用 Git、GitHub Issues、diff、review、commit、push 和上游同步流程，把 Codex 产出变成可检查、可追溯、可维护的学习闭环。

## 2. 背景和动机

Codex 的本地改动最终都落在 Git 工作区里。没有 Git 闭环，就无法确认它改了什么、是否覆盖了用户改动、如何回滚、如何复盘、如何同步上游。学习 Codex 本身也需要协作结构：fork、学习分支、issue 看板、feature docs、实验记录、提交历史。

用户的目标是通过 fork 学习 Codex 和 agent，同时持续合入最新 Codex。Git 和协作不是附属能力，而是学习体系的骨架：`main` 跟上游，`00-study/codex-agent-learning` 放学习材料，GitHub issues 做手机端过程看板，本地文档沉淀稳定结论。

## 3. 用户目标

- 能安全检查 Codex 产生的 diff。
- 能保护用户已有 dirty changes。
- 能把学习材料提交到学习分支并推送。
- 能用 GitHub issue 跟踪功能学习过程。
- 能同步上游 `openai/codex/main` 到 fork 的 `main`，再 rebase 学习分支。
- 能区分个人学习分支、准备 PR、向上游贡献这几条路径。
- 能使用 `/diff`、`/review` 和人工 Git 命令形成验证闭环。

## 4. 适用场景

- 所有会修改文件的 Codex 任务。
- 学习文档和实验记录的日常提交。
- 上游 Codex 更新后的学习分支维护。
- GitHub issue 摘要和状态同步。
- 对本地变更做 review。
- 准备向 fork 或上游提交 PR。

不适合把未验证的 Codex 改动直接推到长期分支，更不适合绕过 diff 和测试。

## 5. 功能边界

- 本阶段以个人 fork 和学习分支为主，不默认向 `openai/codex` 提 PR。
- `main` 只负责跟随上游，不保存学习材料。
- GitHub Issues 保存过程记录和手机端任务状态；本地 feature docs 保存稳定结论。
- `/review` 是本地 review 辅助，不替代人工最终判断。
- Codex app 的 worktrees、cloud review、GitHub trigger 属于后续阶段，可先理解概念。

## 6. 使用约束

- 修改前必须看 `git status --short --branch`。
- 不覆盖用户已有改动；如果任务涉及已有 dirty 文件，先读 diff 再决定如何工作。
- 学习材料只提交到 `00-study/codex-agent-learning`。
- 同步上游前工作区必须干净。
- rebase 后推学习分支使用 `--force-with-lease`，不能用破坏性 reset。
- issue 和文档状态要定期同步。
- 提交消息要表达学习目标或维护动作。

## 7. 用户入口

- `git status --short --branch`：当前分支和 dirty state。
- `git diff`、`git diff --stat`、`git diff --check`：检查变更。
- `git add`、`git commit`、`git push`：提交和推送。
- `git fetch upstream`、`git merge --ff-only upstream/main`、`git rebase main`：同步上游。
- `gh issue list|view|edit|comment|close`：issue 看板。
- `/diff`：TUI 中查看工作区 diff。
- `/review`：本地代码 review。
- `codex apply`：应用 Codex cloud task diff，本阶段只了解。
- `gh pr`：PR 协作，本阶段基础了解。

## 8. 典型工作流

日常学习提交：

1. `git status --short --branch`。
2. 选择一个 feature issue。
3. 修改本地 feature doc 或实验记录。
4. `git diff --check`。
5. 检查 diff 内容。
6. 更新对应 issue 摘要。
7. `git add` 相关文件。
8. `git commit -m "..."`。
9. `git push origin 00-study/codex-agent-learning`。

同步上游：

1. 确认学习分支工作区干净。
2. 切到 `main`。
3. `git fetch upstream`。
4. `git merge --ff-only upstream/main`。
5. `git push origin main`。
6. 切回学习分支。
7. `git rebase main`。
8. 解决冲突并验证学习文档。
9. `git push --force-with-lease`。

本地 review：

1. 完成修改。
2. `/diff` 或 `git diff`。
3. 运行必要验证。
4. `/review` 检查风险。
5. 处理高价值发现。
6. 再次 diff 和验证。

## 9. 最佳实践

- 小步提交，每个提交只表达一个学习结构变化。
- 每次最终回答前检查 status，避免留下半成品。
- issue 顶部摘要保持一屏可读，长结论放本地详细文档。
- 本地文档和 issue 分工固定：docs 是稳定知识，issue 是过程和下一步。
- 上游同步命令只维护在 `maintenance.md`，避免多处漂移。
- 对 GitHub issue 远端更新后，本地 checks 要验证 link 和占位符。
- 提交前运行 `git diff --check`，文档任务也要跑。

## 10. 常见错误

- 在 `main` 写学习材料。
- rebase 前工作区不干净。
- 用 `git add .` 把无关文件提交进去。
- 没看 diff 就 commit。
- issue 写成长文档，手机端反而难用。
- 本地文档更新后忘记 issue 摘要链接。
- 同步上游流程复制到多个文件，后续不一致。
- Codex 生成代码后不运行验证，只相信 summary。

## 11. 风险和限制

- rebase 会重写学习分支历史，需要 `--force-with-lease`，必须确认远端没有他人新提交。
- GitHub issue 是远端状态，本地 commit 不会自动同步 issue。
- fork 的 issue 和上游 issue 是分开的；fork 后可以看到上游公开 issues，但 fork 自己的 issues 独立管理。
- GitHub API/gh 认证可能失效，需要单独排查。
- `/review` 只覆盖给定 diff，不保证所有业务风险。

## 12. 验证方式

本地检查：

```bash
git status --short --branch
git diff --check
git diff --stat
```

issue 检查：

```bash
gh issue list --repo liqiangcc/codex --label feature-map --state open --limit 20
```

状态一致性：

- `feature-map.md` 功能索引状态。
- 对应 issue 顶部摘要状态。
- issue labels 和 open/closed 状态。
- feature doc 是否链接到 issue。

提交后：

```bash
git log --oneline -5
git status --short --branch
```

## 13. 最小练习

完成一次学习文档提交：

1. 修改一个 feature doc。
2. 运行 `git diff --check`。
3. 检查 `git diff --stat`。
4. 更新对应 issue 摘要。
5. 提交并推送。
6. 在 issue #10 记录 commit hash、验证命令、下一步。

## 14. 进阶练习

- 模拟上游同步流程，但在执行前先用 `git status` 和 `git branch -vv` 检查状态。
- 对一次本地文档改动运行 `/review`，观察 review 对文档任务是否有价值。
- 用 `gh issue edit` 批量更新 10 个 feature issue 摘要，验证手机端可读性。
- 创建一个临时 branch 做实验，完成后删除，比较和长期学习分支的区别。
- 阅读 Codex app worktrees 概念，理解它和 CLI 当前学习流程的关系。

## 15. 实验设计

实验目标：验证“feature docs + mobile issue + git commit”的闭环是否可维护。

样本：

- 选择 2 个功能。
- 本地详细文档写稳定知识。
- issue 顶部写当前任务和下一步。
- 提交后再从手机查看 issue。

观察字段：

- 手机一屏是否能看懂当前状态。
- 是否能从 issue 回到详细文档。
- 是否能从详细文档回到 issue。
- commit 是否只包含相关文件。
- 检查命令是否足够发现漂移。

产出：

- 更新 `maintenance.md` 的维护规则。
- 更新 `checks.md` 的检查命令。

## 16. 相关文档

- OpenAI Codex manual：Review。
- OpenAI Codex manual：Codex code review in GitHub。
- OpenAI Codex manual：Worktrees。
- OpenAI Codex manual：Open Source。
- 本仓库：`docs/contributing.md`。
- 本仓库：`study-notes/maintenance.md`。
- 本仓库：`study-notes/checks.md`。

## 17. 源码入口

- `codex-rs/git-utils/src/`：Git 工具函数。
- `codex-rs/core/src/git_info_tests.rs`：Git 信息相关测试。
- `codex-rs/core/src/session/review.rs`：session review 入口。
- `codex-rs/core/src/tasks/review.rs`：review task。
- `codex-rs/prompts/src/review_request.rs`：review prompt 生成。
- `codex-rs/cli/src/main.rs`：review 命令入口相关。
- `codex-rs/cloud-tasks/src/`：cloud task 相关入口，后续阶段阅读。

## 18. 待解决问题

- Codex 如何识别哪些 dirty changes 是用户已有改动？
- `/diff`、app review pane、Git diff 的数据来源和展示范围有什么差别？
- `/review` 的 prompt 和普通“帮我 review”有何不同？
- Codex GitHub review 如何读取 `AGENTS.md` 的 review guidelines？
- 学习分支长期 rebase 后，GitHub issue 链接是否需要调整？

## 19. 当前结论

Git 和协作是 Codex 学习体系的闭环外壳。阶段 1 的固定流程是：先看状态，再改文档或实验，再检查 diff 和验证，再更新 issue，再提交推送。稳定知识留在本地详细文档，过程状态留在 GitHub issue，`main` 始终只跟上游。
