# 学习体系维护手册

这份手册用于维护 Codex 学习分支、功能地图、详细功能文档、GitHub Issues 和实验记录，避免学习材料随着上游更新和个人记录增长而失控。

## 维护目标

- `main` 持续跟随 `openai/codex/main`。
- `00-study/codex-agent-learning` 保留个人学习材料，并定期 rebase 到最新 `main`。
- GitHub Issues 保持可行动，适合手机查看。
- `feature-map.md` 保持索引化、稳定、不过度发散。
- `study-notes/features/*.md` 保存详细、稳定、可复用的功能理解。
- 实验记录能复现，结论能追溯到文档、源码或命令输出。

## 维护节奏

### 每次学习前

1. 确认当前分支：
   ```bash
   git status --short --branch
   ```
2. 如果要基于最新 Codex 学习，先同步上游：
   ```bash
   git checkout main
   git fetch upstream
   git merge --ff-only upstream/main
   git push origin main

   git checkout 00-study/codex-agent-learning
   git rebase main
   git push --force-with-lease
   ```
3. 打开 `feature-map.md`、对应详细功能文档和对应 GitHub issue，确认本轮只推进一个功能或一个问题。

### 每次学习中

- 零散想法、手机端补充、未验证问题先写到 GitHub issue。
- 稳定结论、结构化解释、最佳实践再写回 `study-notes/features/*.md`。
- `feature-map.md` 只同步功能状态、issue 链接、详细文档链接和阶段覆盖矩阵。
- 可复现实验写到 `experiments/YYYYMMDD-topic/`。
- 遇到需要读源码的问题，单独开 `[Source Study] ...` issue。

### 每次学习后

1. 更新对应 issue 的摘要：
   - 当前结论
   - 下一步
   - 是否需要实验或源码阅读
   - 详细文档链接
2. 更新对应 `study-notes/features/*.md`。
3. 如状态变化，更新 `feature-map.md` 中对应功能状态。
4. 如果有实验，确认实验目录包含目的、步骤、观察和结论。
5. 提交并推送学习分支。

### 每周复盘

- 检查 open issues，关闭已经完成或过期的 issue。
- 把 issue 中稳定内容整理回对应详细功能文档。
- 给需要实验的 issue 加 `need-experiment`。
- 给需要源码阅读的 issue 加 `need-source-reading`。
- 检查阶段 1 功能覆盖清单是否需要补充新功能。
- 检查详细功能文档中的源码入口是否仍存在。

### 上游明显更新后

上游更新可能改变命令、配置、功能边界或文档。更新后执行：

1. 同步并 rebase 学习分支。
2. 查看上游 `README.md`、`docs/`、CLI command reference 相关变化。
3. 更新 `study-notes/features/*.md` 中受影响的功能结论。
4. 必要时更新 `feature-map.md` 的覆盖矩阵。
5. 在对应 issue 中记录“上游更新影响”。

## Issue 维护规则

- 一个 issue 只跟踪一个功能或一个源码问题。
- 功能学习 issue 使用 `[Feature Study] ...`。
- 源码阅读 issue 使用 `[Source Study] ...`。
- 实验 issue 使用 `[Experiment] ...`，仅当实验较大或跨多个功能时创建。
- issue 顶部摘要必须保持最新，方便手机查看。
- issue 顶部摘要必须包含对应详细文档链接。
- issue 正文只保留当前状态、当前任务、过程记录、待验证问题和完成条件；不要把完整功能文档复制进 issue。
- 完成后关闭 issue，并在 `feature-map.md` 标记 `Done`。
- 新建功能学习 issue 时优先使用 [Feature Study issue 模板](issue-templates/feature-study.md)，不要修改上游 `.github/ISSUE_TEMPLATE/`。

## 文档维护规则

- `feature-map.md` 只放功能索引、阶段覆盖矩阵和模板说明，不放长聊天记录。
- `study-notes/features/*.md` 保存每个功能的详细稳定结论，章节顺序按“理解 -> 使用 -> 验证 -> 深入”组织。
- `learning-method.md` 只记录方法，不记录具体功能内容。
- `stage-1-use-codex-well.md` 只记录阶段目标、边界、闭环和完成标准。
- `maintenance.md` 只记录维护节奏和规则。
- GitHub issue 只记录过程状态，不复制完整详细文档。

### 详细功能文档维护规则

每个 `study-notes/features/*.md` 至少包含：

- 背景和动机。
- 用户目标。
- 适用场景。
- 功能边界。
- 使用约束。
- 用户入口。
- 典型工作流。
- 最佳实践。
- 常见错误。
- 风险和限制。
- 验证方式。
- 最小练习。
- 进阶练习。
- 实验设计。
- 相关文档。
- 源码入口。
- 待解决问题。
- 当前结论。

详细文档可以持续变长，但每次更新都要保证当前结论可读、练习可执行、源码入口仍存在。

## 状态更新规则

功能索引、详细功能文档和 issue 状态保持一致：

- `Todo`：还未开始。
- `In Progress`：正在学习，有明确下一步。
- `Need experiment`：当前结论需要实验验证。
- `Need source reading`：需要进入源码确认实现。
- `Done`：已经完成阶段 1 成功标准。

如果 issue、详细功能文档和 `feature-map.md` 状态不一致，以 issue 当前状态为过程事实，以详细功能文档为稳定结论，以 `feature-map.md` 为索引结果，复盘时同步三边。

### 状态一致性检查

每周复盘时检查四件事：

1. `feature-map.md` 功能索引中的状态。
2. 对应 GitHub issue 顶部摘要中的状态。
3. 对应详细功能文档中的当前结论和待解决问题。
4. issue 标签和 open/closed 状态。

处理规则：

- issue 仍在推进时，issue 保持 open，功能索引状态不能是 `Done`。
- 功能索引标记 `Done` 时，对应 issue 必须关闭或加 `done` 标签。
- issue 带 `need-experiment` 或 `need-source-reading` 标签时，功能索引状态也应同步为对应状态。
- 如果状态冲突，先更新 issue 摘要，再更新详细功能文档，最后更新 `feature-map.md`。

## 过期处理

以下情况视为可能过期：

- 上游文档或命令发生变化。
- issue 超过两周没有更新。
- 功能索引引用的详细文档不存在。
- 详细功能文档引用的命令、配置或源码路径已经变化。
- 实验步骤无法复现。
- 结论没有文档、源码或命令输出依据。

处理方式：

1. 标记 issue 为 `need-experiment` 或 `need-source-reading`。
2. 在 issue 中写清过期原因。
3. 更新或删除详细功能文档中不可靠的结论。
4. 必要时更新 `feature-map.md` 的覆盖矩阵和状态。

## 质量检查命令

```bash
git status --short --branch
git diff --check
test -d study-notes/features
find study-notes/features -type f | wc -l
rg 'TB[D]|待补[充]' study-notes
rg "最小练习|进阶练习|实验设计|验证方式|源码入口|当前结论" study-notes/features
gh issue list --repo liqiangcc/codex --label feature-map --limit 20
```

占位词不应出现在阶段 1 功能文档和 issue 摘要中。如果确实需要表达未知项，写成具体的“待解决问题”。

更完整的检查项见 [学习检查清单](checks.md)。
