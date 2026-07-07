# 学习体系维护手册

这份手册用于维护 Codex 学习分支、功能地图、GitHub Issues 和实验记录，避免学习材料随着上游更新和个人记录增长而失控。

## 维护目标

- `main` 持续跟随 `openai/codex/main`。
- `00-study/codex-agent-learning` 保留个人学习材料，并定期 rebase 到最新 `main`。
- GitHub Issues 保持可行动，适合手机查看。
- `feature-map.md` 保持结构化、稳定、不过度发散。
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
3. 打开 `feature-map.md` 和对应 GitHub issue，确认本轮只推进一个功能或一个问题。

### 每次学习中

- 零散想法、手机端补充、未验证问题先写到 GitHub issue。
- 稳定结论、结构化解释、最佳实践再写回 `feature-map.md`。
- 可复现实验写到 `experiments/YYYYMMDD-topic/`。
- 遇到需要读源码的问题，单独开 `[Source Study] ...` issue。

### 每次学习后

1. 更新对应 issue 的摘要：
   - 当前结论
   - 下一步
   - 是否需要实验或源码阅读
2. 更新 `feature-map.md` 中对应功能卡片。
3. 如果有实验，确认实验目录包含目的、步骤、观察和结论。
4. 提交并推送学习分支。

### 每周复盘

- 检查 open issues，关闭已经完成或过期的 issue。
- 把 issue 中稳定内容整理回 `feature-map.md`。
- 给需要实验的 issue 加 `need-experiment`。
- 给需要源码阅读的 issue 加 `need-source-reading`。
- 检查阶段 1 功能覆盖清单是否需要补充新功能。

### 上游明显更新后

上游更新可能改变命令、配置、功能边界或文档。更新后执行：

1. 同步并 rebase 学习分支。
2. 查看上游 `README.md`、`docs/`、CLI command reference 相关变化。
3. 更新 `feature-map.md` 中受影响的功能卡片。
4. 在对应 issue 中记录“上游更新影响”。

## Issue 维护规则

- 一个 issue 只跟踪一个功能或一个源码问题。
- 功能学习 issue 使用 `[Feature Study] ...`。
- 源码阅读 issue 使用 `[Source Study] ...`。
- 实验 issue 使用 `[Experiment] ...`，仅当实验较大或跨多个功能时创建。
- issue 顶部摘要必须保持最新，方便手机查看。
- 完成后关闭 issue，并在 `feature-map.md` 标记 `Done`。
- 新建功能学习 issue 时优先使用 [Feature Study issue 模板](issue-templates/feature-study.md)，不要修改上游 `.github/ISSUE_TEMPLATE/`。

## 文档维护规则

- `feature-map.md` 只放稳定结构化结论，不放长聊天记录。
- `learning-method.md` 只记录方法，不记录具体功能内容。
- `stage-1-use-codex-well.md` 只记录阶段目标、边界、闭环和完成标准。
- `maintenance.md` 只记录维护节奏和规则。
- 如果一个功能卡片超过大约 200 行，拆出独立文件并在 `feature-map.md` 链接。

## 状态更新规则

功能卡片和 issue 状态保持一致：

- `Todo`：还未开始。
- `In Progress`：正在学习，有明确下一步。
- `Need experiment`：当前结论需要实验验证。
- `Need source reading`：需要进入源码确认实现。
- `Done`：已经完成阶段 1 成功标准。

如果 issue 和 `feature-map.md` 状态不一致，以 issue 当前状态为过程事实，以 `feature-map.md` 为整理结果，复盘时同步两边。

### 状态一致性检查

每周复盘时检查三件事：

1. `feature-map.md` 功能索引中的状态。
2. 对应 GitHub issue 顶部摘要中的状态。
3. issue 标签和 open/closed 状态。

处理规则：

- issue 仍在推进时，issue 保持 open，功能卡片状态不能是 `Done`。
- 功能卡片标记 `Done` 时，对应 issue 必须关闭或加 `done` 标签。
- issue 带 `need-experiment` 或 `need-source-reading` 标签时，功能卡片状态也应同步为对应状态。
- 如果状态冲突，先更新 issue 摘要，再更新 `feature-map.md`。

## 过期处理

以下情况视为可能过期：

- 上游文档或命令发生变化。
- issue 超过两周没有更新。
- 功能卡片引用的源码路径不存在。
- 实验步骤无法复现。
- 结论没有文档、源码或命令输出依据。

处理方式：

1. 标记 issue 为 `need-experiment` 或 `need-source-reading`。
2. 在 issue 中写清过期原因。
3. 更新或删除 `feature-map.md` 中不可靠的结论。

## 质量检查命令

```bash
git status --short --branch
git diff --check
rg "TBD|待补充" study-notes
gh issue list --repo liqiangcc/codex --label feature-map --limit 20
```

`TBD` 和 `待补充` 可以存在，但每次复盘时应该确认它们是否仍然合理。

更完整的检查项见 [学习检查清单](checks.md)。
