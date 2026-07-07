# 第一阶段：用好 Codex CLI

第一阶段先从使用者视角建立行为模型，再进入源码实现。目标不是一次性读完源码，而是能稳定地用 Codex 完成真实开发任务，并知道如何控制范围、权限和验证结果。

## 阶段目标

熟练使用 Codex CLI 完成本地仓库中的开发、解释、调试、验证和复盘任务。

完成第一阶段时，应能够做到：

- 把一个模糊需求拆成 Codex 可以执行的清晰任务。
- 判断什么时候让 Codex 直接执行，什么时候先让它计划。
- 给出足够上下文，同时避免把任务范围放得太大。
- 正确使用 sandbox、approval、network、AGENTS.md、config 和 slash commands。
- 要求 Codex 运行合适的验证，并自己检查 diff、测试结果和残留风险。
- 把一次使用经验沉淀成功能卡片、实验记录或下一步源码问题。

## 阶段边界

第一阶段聚焦：

- Codex CLI 的本地使用。
- 本地仓库中的读代码、改代码、跑命令、跑测试和 review。
- 任务表达、上下文管理、权限安全、配置、项目约定和验证闭环。
- GitHub Issues 作为移动端学习看板。

第一阶段暂不深入：

- IDE extension、Codex app、Codex cloud 的完整工作流。
- App server、SDK、protocol、remote-control 的实现细节。
- 大规模源码改造或准备向上游贡献的代码变更。

## 使用约束

- `main` 只同步 `openai/codex/main`，不放学习内容。
- 学习文档、实验和 issue 只围绕 `00-study/codex-agent-learning` 分支。
- 每个实验只验证一个机制，避免把多个变量混在一起。
- 涉及权限、安全、沙箱、网络和文件删除的行为必须先写清风险。
- 稳定结论沉淀到 `study-notes/feature-map.md`，过程记录放在 GitHub Issues。
- 后续维护节奏和状态同步规则见 [学习体系维护手册](maintenance.md)。

## 闭环流程

每个功能按下面流程推进：

1. 建卡片：在 `feature-map.md` 中创建或更新功能卡片。
2. 建 issue：在 GitHub Issues 创建同名 `[Feature Study] ...` issue。
3. 补摘要：先填顶部摘要，保证手机上一屏能看懂当前状态。
4. 查资料：阅读官方手册和仓库 docs，补背景、边界和入口。
5. 跑场景：设计 1-2 个真实使用场景，记录 prompt、命令和观察结果。
6. 做实验：必要时在 `experiments/YYYYMMDD-topic/` 保存最小实验。
7. 写结论：把稳定结论整理回功能卡片。
8. 转源码：如果需要深入实现，另开 `[Source Study] ...` issue。
9. 完成关闭：满足完成标准后关闭 issue，并把卡片状态标记为 `Done`。

## Issue 用法

GitHub Issues 作为移动端友好的过程看板：

- 随手记录问题、链接、截图说明、观察和下一步。
- 用标签区分状态：`study`、`feature-map`、`stage-1`、`need-experiment`、`need-source-reading`、`done`。
- issue 里允许粗糙记录，文档里只保留整理后的稳定结论。
- 功能学习和源码阅读分开建 issue，避免一个 issue 过长。

## 完成标准

一个功能卡片标记为 `Done` 前，至少满足：

- 能用一句话解释该功能是什么。
- 写清适合和不适合的使用场景。
- 写清边界、约束、风险和验证方式。
- 至少完成一次真实使用或最小实验。
- 记录相关文档和后续源码入口。
- 留下结论或明确的下一步问题。

## 每轮学习输出

每轮学习至少留下一个结果：

- 更新一个功能卡片。
- 更新一个 GitHub issue。
- 新增一个实验记录。
- 新增一个源码阅读问题。
- 关闭一个已完成的学习 issue。
