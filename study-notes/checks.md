# 学习检查清单

这份清单用于每次学习后或每周复盘时检查学习体系是否仍然闭环。

## 基础检查

```bash
git status --short --branch
git diff --check
```

预期：

- 当前分支是 `00-study/codex-agent-learning`，或你明确知道自己正在同步 `main`。
- 工作区没有意外未提交改动。
- `git diff --check` 没有空白错误。

## 文档检查

```bash
test -f study-notes/stage-1-use-codex-well.md
test -f study-notes/feature-map.md
test -f study-notes/maintenance.md
test -f study-notes/checks.md
test -f study-notes/issue-templates/feature-study.md
test -d study-notes/features
find study-notes/features -type f | wc -l
```

```bash
rg "阶段 1|阶段 2|maintenance.md|checks.md" study-notes/README.md
rg "状态一致性检查|issue.*feature-map" study-notes/maintenance.md
rg "详细文档|features/" study-notes/feature-map.md study-notes/README.md study-notes/maintenance.md
rg "最小练习|进阶练习|实验设计|验证方式|源码入口|当前结论" study-notes/features
```

预期：

- README 先引导阶段 1，再引导阶段 2 源码阅读。
- 同步上游的完整命令只在维护手册里维护。
- issue 模板和检查清单都有入口。
- `study-notes/features/` 下有 10 个详细功能文档。
- 每个详细功能文档都包含练习、实验、验证、源码入口和当前结论。

## Issue 检查

```bash
gh issue list --repo liqiangcc/codex --label feature-map --state open --limit 20
```

预期：

- 阶段 1 功能学习 issue 共有 10 个，除非已经完成并关闭。
- 每个 issue 都有 `study`、`feature-map`、`stage-1` 标签。

检查 issue 是否还残留占位符：

```bash
gh issue list --repo liqiangcc/codex --label feature-map --state open --json number,title,body \
  --jq '.[] | {number,title,placeholders: ((.body | split("待补" + "充") | length) - 1)}'
```

预期：

- 已经初始化过的功能 issue 中 `placeholders` 为 `0`。

检查 issue 是否包含详细文档链接：

```bash
gh issue list --repo liqiangcc/codex --label feature-map --state open --json number,title,body \
  --jq '.[] | {number,title,has_detail: (.body | contains("详细文档"))}'
```

预期：

- 每个 open feature issue 的 `has_detail` 为 `true`。

## 状态一致性检查

人工检查：

1. `feature-map.md` 功能索引状态。
2. 对应 issue 顶部摘要状态。
3. 对应详细功能文档中的“当前结论”和“待解决问题”。
4. issue 标签和 open/closed 状态。

规则：

- `Need experiment` 状态应有 `need-experiment` 标签。
- `Need source reading` 状态应有 `need-source-reading` 标签。
- `Done` 状态应关闭 issue，或至少加 `done` 标签。
- open issue 不应对应 `Done` 的功能卡片。
- issue 摘要、详细功能文档和 `feature-map.md` 的下一步不应互相矛盾。

## 同步命令去重检查

预期：

- 完整同步命令只保留在 `study-notes/maintenance.md`。
- 其他文档只链接维护手册，不复制命令。
- 复盘时可用文本搜索确认没有其他文档复制维护手册里的同步命令。

## 上游漂移检查

当上游 Codex 明显更新后：

先按 [学习体系维护手册](maintenance.md#每次学习前) 同步上游和学习分支。

然后检查：

```bash
git diff HEAD~1 -- README.md docs codex-rs/cli codex-cli 2>/dev/null || true
```

如果 CLI 行为、配置、权限或文档入口变化，更新对应详细功能文档、feature issue 和 `feature-map.md`。
