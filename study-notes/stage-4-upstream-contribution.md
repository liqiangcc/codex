# 阶段 4：上游协作与贡献

## 阶段目标

把源码理解转化为可被上游维护者审阅的贡献能力：从真实问题出发，完成复现、根因、最小变更、测试、review、PR 和反馈修正闭环。目标不是提交数量，而是建立可重复的高质量贡献方法。

## 前置条件

- 阶段一至三已完成，能够说明用户行为、核心源码和扩展边界。
- `main` 与 `openai/codex/main` 同步，学习分支保持独立。
- 贡献前重新阅读根 `AGENTS.md`、`docs/contributing.md` 和相关目录 guidance。

## 主题矩阵

| 顺序 | 主题 | 必须回答的问题 | 证据入口 | 最小任务 | 计划 issue |
| --- | --- | --- | --- | --- | --- |
| 1 | 上游同步 | 如何保持 `main`、学习分支和贡献分支职责分离？ | `study-notes/maintenance.md`、`docs/contributing.md` | 完成一次无冲突同步演练并记录分支图 | [#18](https://github.com/liqiangcc/codex/issues/18) |
| 2 | 问题诊断 | 这是 bug、配置问题、预期行为还是缺少信息？ | issue、日志、最小复现、相关测试 | 为一个真实问题写诊断报告 | [#19](https://github.com/liqiangcc/codex/issues/19) |
| 3 | 根因与设计 | 哪个最小代码路径导致行为，变更边界是什么？ | 阶段二源码链路、breaking-change 清单 | 写出根因、候选方案和取舍 | [#20](https://github.com/liqiangcc/codex/issues/20) |
| 4 | 实现与测试 | 如何用最小 diff 修复并覆盖用户行为？ | crate tests、integration tests、snapshots、CI | 完成一个小修复及相关测试 | [#21](https://github.com/liqiangcc/codex/issues/21) |
| 5 | Review 与 PR | 如何让 reviewer 快速理解影响、证据和风险？ | `.github/pull_request_template.md`、`/review` | 完成本地 review 和 draft PR 描述 | [#22](https://github.com/liqiangcc/codex/issues/22) |
| 6 | 反馈闭环 | 如何处理 CI、review comments、冲突和后续维护？ | checks、review threads、commit history | 处理一轮真实或模拟反馈 | [#23](https://github.com/liqiangcc/codex/issues/23) |

## 学习顺序

1. 固定分支与同步规则，禁止在学习分支直接准备上游提交。
2. 先诊断并证明问题，再进入代码设计。
3. 控制 change size，优先完成最小可审阅阶段。
4. 按受影响 crate 运行格式化、测试和 lint。
5. 本地 review 后再准备 PR，明确用户影响和残留风险。
6. 持续处理 CI 与 review，直到结论稳定。

## 每项完成证据

- 可复制的复现步骤和实际/期望行为。
- 根因对应的源码路径、关键类型/函数和受影响外部接口。
- 变更文件、测试命令、输出摘要和未覆盖风险。
- review 发现及处理结果；没有发现时也记录测试盲区。
- PR 标题/正文、CI 状态、review feedback 和最终结果。

## 阶段产物

- 6 张 `[Contribution Study]` issue。
- 一份可重复的贡献检查清单。
- 至少一个独立贡献分支上的小型、可测试变更。
- 一次完整的 draft PR 或等价的本地模拟 review 包。

## 完成标准

- [x] 六个主题全部完成并留下证据。
- [x] 至少一个问题完成“复现 → 根因 → 修复 → 测试 → review”闭环。
- [x] 对 app-server API、CLI、配置、rollout 恢复等 breaking surfaces 做过明确检查。
- [x] 相关格式化、目标 crate 测试和必要 lint 均通过。
- [x] 能处理至少一轮 CI 或 review feedback，不覆盖用户已有改动。

## 验收摘要（2026-07-10）

- fork `main` 已在确认纯 fast-forward 后同步到最终审计时的 `upstream/main` `2b9c050460`；包含本审计提交后，学习分支为 `20 ahead / 0 behind`。
- 独立 worktree/branch `agent/plugin-creator-hooks-validation` 完成真实 bug 的失败复现、根因、最小修复、测试与 review。
- 贡献提交 `53a917a3ac` 已推送到个人 fork，刷新分支 head 为 `d7d109c3f6`、相对 upstream `3 ahead / 0 behind`；Python 7/7、`codex-skills` 1/1、scoped Clippy 与格式化通过，最终 upstream 刷新后目标测试再次通过。
- 已处理一轮模拟 review feedback，补充 plugin archive 逃逸和损坏 JSON 回归。
- 根据 `docs/contributing.md` 的邀请制政策，没有向 upstream 创建未经邀请的 PR；等价 draft PR/review 包见 [阶段四证据索引](contributions/README.md)。
- Issues [#18](https://github.com/liqiangcc/codex/issues/18)–[#23](https://github.com/liqiangcc/codex/issues/23) 已完成并关闭。

## 阶段边界

- 不以学习名义向上游提交未经验证的大改动。
- 不把多个不相关问题塞进同一 PR。
- 不使用 destructive Git 命令处理冲突或清理工作区。
- 不执行 `/usage`、登出、账号切换或额度重置命令。
