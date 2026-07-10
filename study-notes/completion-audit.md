# 四阶段完成总验收

验收日期：2026-07-10。

权威范围来自 [学习阶段目标](stage-goals.md) 及四份阶段执行文件。阶段文件明确排除 IDE/App/Cloud/SDK 的完整实现、生产系统和未经邀请的 upstream PR，因此“全部完成”指四阶段定义的 CLI 使用、核心链路、扩展机制和贡献闭环全部满足，不把边界外产品面伪装成已穷尽。

## 总体结论

| 阶段 | 明确要求 | 当前权威证据 | 结论 |
| --- | --- | --- | --- |
| 1：用好 CLI | 10 张功能卡均有文档、实验、issue 和结论；能完成范围/权限/配置/扩展/Git 闭环 | `feature-map.md` 10/10 Done；10 份功能文档章节齐全且无占位符；#1–#10 CLOSED；真实 TUI、exec、权限、配置、diff、Git 记录 | 通过 |
| 2：核心源码链路 | CLI → turn → sampling → tools → history/compaction/MCP；每段输入/输出/不变量；行为交叉验证 | `stage-2-source-reading.md` 的逐主题矩阵；最新 upstream 上重新定位全部关键符号；#11 CLOSED | 通过 |
| 3：扩展机制 | 扩展层级决策；隔离成功/失败；权限、生命周期、上下文和回滚；6 主题与可安装扩展 | 6 份扩展笔记；Skill/Plugin validator；MCP 与 Hook 实验；`codex-hooks` 120/120；Plugin remove/re-add 新线程验证；#12–#17 CLOSED | 通过 |
| 4：贡献闭环 | 最新 main 独立分支；真实问题复现/根因/修复/测试；review/PR 描述/feedback | fork/学习/贡献分支均 behind 0；真实 validator 漂移补丁；Python 7/7、`codex-skills` 1/1；draft PR 与反馈记录；#18–#23 CLOSED | 通过 |

## 阶段 1 逐项证据

- `study-notes/features/` 恰有 10 份一级功能文档；每份都有背景、目标、场景、边界、约束、入口、工作流、最佳实践、错误、风险、验证、最小/进阶练习、实验、文档/源码入口、待解决问题和当前结论。
- 自动检查没有发现未完成占位标记；源码路径检查没有发现不存在的 `codex-rs/...` 入口。
- [功能地图](feature-map.md) 10 项全部为 Done，并分别链接 #1–#10 和对应详细文档。
- GitHub #1–#10 均为 CLOSED；每个正文顶部均有 Done、当前结论、下一步和稳定文档链接。
- #1 和 #2 的补充 PTY 实验实际观察 `/status`、`/plan`、`/permissions`、`/diff`、`/compact`；没有执行 `/usage`、登出、账户切换或额度重置。
- 阶段一的完成深度以 [功能覆盖清单](feature-map.md#阶段-1-功能覆盖清单) 为准；浅用/可选/边界外项目没有被误标为源码精通。

## 阶段 2 逐项证据

- 当前学习分支已合入 `upstream/main` `2b9c050460`，并在该源码上重新定位：`cli_main`、`Op::UserTurn` handler、`RegularTask`、`run_turn`、`run_sampling_request`、`try_run_sampling_request`、`ToolRouter`、`ToolOrchestrator`、`runtime_mcp_config`、`mcp_runtime_for_step` 和 `refresh_mcp_servers_now`。
- [逐主题验收矩阵](stage-2-source-reading.md#逐主题验收矩阵) 对 6 个主题分别记录输入、输出、不变量、行为/测试交叉证据与阶段边界。
- 端到端链路明确包含 tool output 写回 history 后继续 sampling，而不是只描述第一次模型请求。
- approval/sandbox、compaction 和 step-scoped MCP runtime 的介入点均有源码入口与阶段一/三行为证据。
- #11 为 CLOSED，顶部摘要与详细文档链接齐全。

## 阶段 3 逐项证据

- [扩展决策](extensions/extension-decision.md) 对 prompt、guidance、Skill、MCP、Hook 和 Plugin 做最轻层级选择，最终组合为 `Plugin(Skill + MCP)`；Hook 只保留隔离实验。
- `$observe-codex-study` 由 skill-creator 初始化，Skill validator 与 Plugin validator 当前均通过；新线程 `/skills` 搜索可命中显示名。
- MCP 固定启动 `approval=never`、`sandbox=read-only` 的 Codex 子进程，不接受 arbitrary executable/arguments；成功调用、未知 session 失败和 finally stop 均已观察。
- Stop Hook fixture 有效输入退出 0、错误事件退出 2；当前最新 upstream 上 `just test -p codex-hooks` 为 120/120。
- Plugin cachebuster 版本为 `0.1.0+codex.20260710145859`。实际 remove 后 `plugin list` 为 not installed，新线程搜索不到 Skill；re-add 后为 installed/enabled，新线程重新命中 Skill；所有子进程显式 stop。
- 6 张 Extension Study issue（#12–#17）全部 CLOSED，摘要字段和详细文档链接完整。
- 敏感数据扫描覆盖学习笔记、实验和 personal plugin source，没有发现 token、email、private key 或真实 UUID。

## 阶段 4 逐项证据

- `origin/main` 已纯 fast-forward 到 `upstream/main` `2b9c050460`；学习分支通过普通 merge 吸收 main，最终为 `20 ahead / 0 behind`，没有 rebase/force/reset。
- 独立 worktree `/root/codex-stage4` 的贡献分支为 `agent/plugin-creator-hooks-validation`；刷新 head `d7d109c3f6` 相对 upstream 为 `3 ahead / 0 behind`。
- 最新 upstream 仍存在真实问题：core 和参考示例支持 Plugin `hooks`，bundled validator 的 allowlist 仍拒绝它，参考末尾也保留矛盾说明。
- 修复提交 `53a917a3ac` 只改变 validator、独立回归测试和对应参考，当前相对 upstream diff 为 3 files、214 additions、3 deletions。
- 修复前测试明确失败；修复和 review feedback 后 Python 7/7、`codex-skills` 1/1。scoped Clippy 与格式化通过；upstream 刷新后目标测试再次通过。
- Breaking-surface 检查确认不触及 app-server API、raw response events、CLI 参数、配置加载或 rollout 恢复。
- [draft PR](contributions/draft-pr.md) 含 What/Why/How/Tests/Risk/Policy；一轮 review feedback 增加 archive escape 与 invalid JSON 覆盖。
- `docs/contributing.md` 当前要求外部代码 PR 先获邀请；没有向 `openai/codex` 创建未经邀请的 PR，个人 fork 分支和本地 draft 包构成阶段计划允许的等价 review 证据。
- 6 张 Contribution Study issue（#18–#23）全部 CLOSED，摘要字段和详细文档链接完整。

## 跨阶段一致性与安全

- GitHub #1–#23 全部 CLOSED；23/23 正文顶部同时具有 Done、当前结论、下一步和详细文档链接。
- 53 份 `study-notes/**/*.md` 的相对链接全部可解析；所有学习笔记中的 `codex-rs/...` 源码入口在当前树中存在。
- [维护手册](maintenance.md) 已把长期学习分支同步改为 fast-forward main + 普通 merge + 正常 push，明确禁止 rebase/force/reset/丢弃式 checkout。
- 未执行 `/usage`、登出、账户切换、额度重置或破坏性 Git 命令；没有把账号输出、token、session ID 或 secret 写入仓库/issue。
- personal Plugin 最终恢复为 installed/enabled，仍固定 read-only；两个 Git worktree 在最终提交前均无未知或用户遗留改动。

## 最终判定

四个阶段的显式目标、编号主题、产物、issue、实验、测试、边界和维护闭环均有当前证据支持，没有剩余必做项。后续 upstream 更新属于重新验证触发器，不是本次阶段完成的欠项。
