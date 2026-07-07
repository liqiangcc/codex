# 功能：权限和安全

## 1. 一句话说明

用 sandbox、approval、permissions、network、execpolicy 和 hooks 控制 Codex 可以读写什么、运行什么、何时必须停下来请求确认。

## 2. 背景和动机

Codex 是 coding agent，不只是聊天模型。它会读文件、改文件、运行命令、调用工具、访问外部系统。没有清晰权限边界，就会出现误删文件、改错目录、泄露敏感数据、执行危险命令、访问不该访问的网络资源等风险。

安全能力的核心是两层：sandbox 规定技术边界，approval 规定跨边界时是否需要用户确认。第一阶段必须先理解这两个概念，否则后续使用 `codex exec`、MCP、hooks、plugins、web search、Git 操作时都会缺少判断依据。

## 3. 用户目标

- 能解释 sandbox mode、approval policy、permission profiles、network access 的区别。
- 能判断当前任务需要 read-only、workspace-write 还是 danger-full-access。
- 能用 `/permissions` 和 `/status` 检查并调整当前会话权限。
- 能识别哪些操作必须先审批、哪些应该直接拒绝。
- 能设计最小权限实验验证边界。
- 能在 issue 或日志中记录权限问题而不泄露敏感信息。

## 4. 适用场景

- 所有会修改文件的任务。
- 所有会运行 shell 命令、测试、构建、包管理器的任务。
- 需要访问网络、下载依赖、查 live web 的任务。
- 需要读写当前仓库之外路径的任务。
- 使用 MCP、apps/connectors、plugins、hooks 的任务。
- 自动化和 `codex exec` 脚本。

不适合用“直接开 full access”解决所有卡点。full access 只适合外部环境已经隔离且用户明确接受风险的场景。

## 5. 功能边界

- Sandbox 控制命令和工具执行的技术访问边界。
- Approval policy 控制 Codex 何时向用户请求确认。
- Permission profiles 是更细粒度、可复用的文件系统和网络权限配置。
- Execpolicy/rules 控制命令策略：允许、提示或阻止特定命令。
- Hooks 可以在生命周期中执行自定义检查，但 hook 本身也有信任边界。
- Web search 工具不等同于 shell 命令网络访问；两者分别配置和评估。

## 6. 使用约束

- 默认从最小权限开始：只读分析用 read-only，普通本地开发用 workspace-write + on-request。
- 不可信仓库、未知脚本、外部贡献代码中不要使用 full access。
- 网络默认关闭时，不要为了“方便”全局开启；能限定域名就限定域名。
- 对 `rm -rf`、凭据访问、系统目录写入、包管理安装、Docker socket、SSH key、浏览器 profile 等高风险动作必须先解释风险。
- Hooks 和 plugins 需要审查来源，非 managed hook 需要 trust 之后才运行。
- Approval 放行不是安全证明，只是用户同意尝试。

## 7. 用户入口

- `--sandbox read-only|workspace-write|danger-full-access`。
- `--ask-for-approval untrusted|on-request|never`。
- `--dangerously-bypass-approvals-and-sandbox` / `--yolo`。
- `/permissions`：交互式切换权限模式。
- `/status`：查看当前 sandbox、approval、writable roots、workspace。
- `config.toml`：`sandbox_mode`、`approval_policy`、`approvals_reviewer`、`sandbox_workspace_write`。
- permission profiles：`:read-only`、`:workspace`、`:danger-full-access` 或自定义 `[permissions.*]`。
- `web_search = "cached"|"live"|"disabled"`。
- `[sandbox_workspace_write].network_access = true`。
- `codex execpolicy` 和 rules。
- `/hooks`：查看、信任或禁用 hooks。
- `--dangerously-bypass-hook-trust`：仅适合已经外部审查 hook 的自动化。

## 8. 典型工作流

只读学习：

1. 使用 read-only 或默认安全模式。
2. 让 Codex 解释代码，不允许修改。
3. 如果需要运行命令，让它先说明命令目的。
4. 记录结论。

普通开发：

1. `workspace-write + on-request`。
2. 限制任务范围在当前仓库。
3. Codex 可以编辑工作区和运行本地命令。
4. 访问仓库外目录或网络时需要审批。
5. 修改后通过 Git diff 和测试验证。

自动化：

1. 用 `codex exec --sandbox read-only` 先分析。
2. 需要写入时显式改为 `workspace-write`。
3. 避免在同一 job 中把 API key 暴露给不可信代码。
4. 保存 patch 或报告，由后续受控步骤处理。

高风险操作：

1. 要求 Codex 先解释目的、影响范围、回滚方式。
2. 使用 approval 或 rules 阻止直接执行。
3. 用户确认后再执行。
4. 执行后立即检查 Git 状态和文件系统影响。

## 9. 最佳实践

- 把权限当作任务输入的一部分写清楚：允许读、允许改、禁止网络、禁止删除。
- 使用 `/status` 做权限事实来源，不凭感觉判断。
- 对新仓库先 read-only 探索，再逐步放开到 workspace-write。
- 需要多目录协作时优先用 `--add-dir` 或 permission profile，不要直接 full access。
- 网络访问按域名和目的最小化；web search 用 cached 模式时也按不可信外部输入处理。
- 对自动化使用 `--ignore-user-config` 或固定 profile，减少个人配置干扰。
- 记录每次权限放开的理由，尤其是网络、外部路径和 destructive 操作。

## 10. 常见错误

- 把 sandbox 和 approval 混为一谈。
- 认为 `approval_policy = "never"` 就意味着可以做任何事；sandbox 仍然限制能力。
- 认为 full access 是“高级模式”，日常任务默认打开。
- 为了跑测试开启网络，却没有确认测试是否真的需要网络。
- 把 web search 和 shell 网络访问混淆。
- 安装 plugin 或启用 hook 后不审查它能做什么。
- 在 issue 中粘贴包含 token 的审批提示、日志或环境变量。

## 11. 风险和限制

- 权限过宽会放大 agent 误操作和 prompt injection 风险。
- 权限过窄会导致任务无法完成，并可能让 Codex 做出不完整结论。
- 网络访问可能触发依赖安装、远端 API 调用或数据外发。
- Hooks、MCP、plugins 会增加外部代码和外部系统的信任面。
- 平台 sandbox 实现不同，macOS、Linux、WSL2、Windows 的行为和依赖不同。
- workspace-write 中 `.git`、`.codex` 等路径可能仍受保护，Git 提交等操作可能需要 approval。

## 12. 验证方式

基础检查：

```text
/status
```

对照实验：

1. read-only 下请求 Codex 修改一个临时文件，应触发限制或审批。
2. workspace-write 下修改当前仓库文件，应在边界内完成。
3. workspace-write 下访问仓库外路径，应需要审批或失败。
4. network off 下执行需要联网的命令，应失败或请求审批。
5. 开启 web search 和开启 shell 网络访问分别观察 transcript 差异。

记录：

- 当前 sandbox。
- 当前 approval policy。
- 是否启用 network。
- 请求动作。
- 结果是允许、审批、拒绝还是失败。

## 13. 最小练习

在一个不会影响业务的临时目录完成：

1. 启动 Codex，运行 `/status`。
2. 切到 read-only，要求 Codex 修改一个文件，观察行为。
3. 切到 Auto/workspace-write，要求 Codex 修改当前仓库内一个学习笔记。
4. 要求 Codex 读取仓库外路径，观察是否需要审批。
5. 在 issue #5 记录每一步的模式、动作、结果。

## 14. 进阶练习

- 写一个最小 execpolicy rule，让某类命令提示或阻止，再用 `codex execpolicy` 评估。
- 配置一个 permission profile：工作区可写、`.env` deny、网络只允许某个域名。
- 开启 `/hooks` 浏览器，检查当前是否有 hooks 需要 trust。
- 用 `web_search = "disabled"` 和 `web_search = "live"` 对比同一个资料查询任务。
- 研究 `approvals_reviewer = "auto_review"` 的触发条件和失败关闭语义。

## 15. 实验设计

实验目标：建立自己的权限选择矩阵。

任务类型：

- 只读解释。
- 文档编辑。
- 跑测试。
- 安装依赖。
- 访问网络 API。
- 读取仓库外文件。

权限组合：

- read-only + on-request。
- workspace-write + on-request。
- workspace-write + never。
- danger-full-access + never。

观察字段：

- 是否能完成任务。
- 是否请求审批。
- 是否触碰预期外路径。
- 是否访问网络。
- 用户是否能理解风险提示。

产出：

- 一张“任务类型 -> 推荐权限”的表。
- 一组“不使用 full access 的替代做法”。

## 16. 相关文档

- OpenAI Codex manual：Agent approvals & security。
- OpenAI Codex manual：Sandbox。
- OpenAI Codex manual：Permissions。
- OpenAI Codex manual：Hooks。
- OpenAI Codex manual：Config basics。
- 本仓库：`docs/sandbox.md`。
- 本仓库：`docs/execpolicy.md`。
- 本仓库：`docs/config.md`。

## 17. 源码入口

- `codex-rs/core/src/sandboxing/`：核心 sandbox 相关逻辑。
- `codex-rs/sandboxing/src/`：sandboxing crate。
- `codex-rs/linux-sandbox/src/`：Linux sandbox 实现。
- `codex-rs/core/src/exec_policy.rs`：核心命令策略。
- `codex-rs/execpolicy/src/`：execpolicy CLI 和规则评估。
- `codex-rs/config/src/permissions_toml.rs`：permission profiles 配置解析。
- `codex-rs/core/src/config/permissions.rs`：core 权限配置。
- `codex-rs/core/src/network_policy_decision.rs`：网络策略决策。
- `codex-rs/hooks/src/`：hooks 引擎、事件和 trust 流程。

## 18. 待解决问题

- sandbox、approval、execpolicy、hook 的实际执行顺序是什么？
- permission profiles 和旧的 `sandbox_mode`/`sandbox_workspace_write` 如何互斥或迁移？
- web search live/cached 和 shell network access 在 transcript 中如何区分？
- workspace-write 下 `.git` 保护如何影响 commit、rebase、stash？
- 自动 approval reviewer 的 denial 如何在 TUI 中暴露给用户？

## 19. 当前结论

权限和安全是用好 Codex 的核心前提。阶段 1 的默认策略是：read-only 用于学习和分析，workspace-write + on-request 用于普通本地开发，full access 只在外部隔离且明确接受风险时使用。
