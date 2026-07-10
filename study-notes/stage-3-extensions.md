# 阶段 3：扩展与运行机制

## 阶段目标

理解 Codex 扩展能力如何被发现、加载、授权和执行，并能为一个重复流程选择最轻量、可维护的实现层级。阶段三不是“把所有东西都做成 plugin”，而是学会在 prompt、`AGENTS.md`、skill、MCP、hook 和 plugin 之间做有证据的选择。

## 前置条件

- 阶段一已完成，能安全使用 CLI、权限和 Git 闭环。
- 阶段二已完成，理解 session、tools、context 和 MCP runtime 的介入点。
- 所有实验默认 read-only 或最小 workspace-write；不使用真实生产凭据。

## 主题矩阵

| 顺序 | 主题 | 必须回答的问题 | 源码入口 | 最小实验 | 计划 issue |
| --- | --- | --- | --- | --- | --- |
| 1 | 扩展决策 | 什么情况下用 prompt、guidance、skill、MCP、hook、plugin？ | `study-notes/features/08-tools-and-extensions.md` | 为 5 个重复流程做扩展层级决策表 | [#12](https://github.com/liqiangcc/codex/issues/12) |
| 2 | Skills | skill 如何发现、匹配、注入指令和依赖？ | `codex-rs/core-skills/`、`codex-rs/skills/` | 创建只含 instructions 的本地 skill 并显式调用 | [#13](https://github.com/liqiangcc/codex/issues/13) |
| 3 | MCP | server/tool 如何进入 step runtime，approval 如何合并？ | `codex-rs/codex-mcp/`、`core/src/session/mcp.rs` | 用只读 MCP 完成一次启动、调用、失败和清理 | [#14](https://github.com/liqiangcc/codex/issues/14) |
| 4 | Hooks | hook 的发现、trust、matcher、事件和失败语义是什么？ | `codex-rs/hooks/`、`core/src/hook_runtime.rs` | 在临时项目验证一个只读 Stop hook | [#15](https://github.com/liqiangcc/codex/issues/15) |
| 5 | Plugins | plugin 如何打包 skills、MCP、apps 并进入 marketplace/cache？ | `codex-rs/core-plugins/`、`codex-rs/plugin/` | 检查一个 personal plugin 的 manifest、安装、升级和禁用边界 | [#16](https://github.com/liqiangcc/codex/issues/16) |
| 6 | 综合扩展 | 一个真实学习流程应该由哪些组件组成？ | 上述模块和 `study-notes/maintenance.md` | 为“观察 TUI 并记录结果”做端到端架构复盘 | [#17](https://github.com/liqiangcc/codex/issues/17) |

## 学习顺序

1. 先做扩展层级决策，明确需求和最轻实现。
2. 从 instruction-only skill 开始，理解上下文注入。
3. 再进入 MCP，理解工具协议与权限边界。
4. 研究 hooks 的生命周期和失败关闭语义。
5. 最后学习 plugin 的打包、缓存和分发。
6. 用一个综合实验检验组合是否过度设计。

## 每项完成证据

每个主题必须留下：

- 一张职责/生命周期图或文字链路。
- 至少两个直接源码入口和一个关键类型或函数。
- 一个最小成功实验和一个预期失败路径。
- 权限、数据、凭据、上下文大小和清理边界。
- 对应 issue 的当前结论、下一步和详细文档链接。

## 阶段产物

- `study-notes/extensions/` 下的主题笔记。
- `experiments/YYYYMMDD-*/` 下的隔离实验。
- 6 张 `[Extension Study]` issue。
- 一个可安装、可禁用、默认最小权限的学习扩展。

## 完成标准

- [x] 六个主题全部完成并有源码与行为证据。
- [x] 能根据需求说明为什么选择某种扩展层级。
- [x] 综合扩展覆盖启动、调用、权限、错误、清理和升级路径。
- [x] 未在仓库或 issue 中保存 token、账号数据或真实 secret。
- [x] 所有实验可关闭或卸载，且不会扩大 Codex 默认权限。

## 验收摘要（2026-07-10）

- 六主题源码与行为记录见 [阶段三证据索引](extensions/README.md)。
- personal plugin 已新增 `$observe-codex-study`，Skill/Plugin 校验通过，并以 cachebuster `0.1.0+codex.20260710145859` 重装。
- 全新只读子 Codex 的 `/skills` 列表可发现新 Skill；MCP 的启动、UI 调用、未知 session 失败和显式 stop 均已验证。
- Stop Hook handler 的有效/无效 fixture 分别退出 0/2，`codex-hooks` 测试 120/120 通过。
- Plugin 已实际执行 remove/re-add：卸载后的新线程搜索不到 Skill，恢复后的新线程可通过 `/skills` 搜索命中，两个子进程均保持 read-only 并显式清理。
- Issues [#12](https://github.com/liqiangcc/codex/issues/12)–[#17](https://github.com/liqiangcc/codex/issues/17) 已完成并关闭。

## 阶段边界

- 不开发面向公开市场的大型产品 plugin。
- 不接入生产系统、付费 API 或真实业务数据。
- 不把 app-server、SDK、IDE、Desktop、Cloud 的完整实现纳入本阶段。
- 不执行 `/usage`、登出、账号切换或额度重置命令。
