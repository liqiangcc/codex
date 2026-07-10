# 功能：安装、登录、启动

## 1. 一句话说明

让 Codex CLI 在本机、目标账号和目标仓库中进入可运行、可诊断、可恢复的基线状态。

## 2. 背景和动机

安装、登录、启动不是一次性的准备工作，而是所有 Codex 学习的地基。后续遇到的很多问题看起来像 agent 行为问题，实际可能来自版本过旧、认证方式不对、配置层级冲突、当前目录不是 Git 仓库、终端环境异常、网络或 sandbox 设置不符合预期。

第一阶段不急着读源码，先要把“当前 Codex 是什么版本、用什么账号、在哪个目录、有哪些权限、能不能正常诊断”变成可重复检查的动作。这样后续学习每个功能时，才能把问题限定在功能本身，而不是被环境漂移干扰。

## 3. 用户目标

- 能在任意目标仓库中启动 Codex CLI。
- 能确认当前安装版本、认证状态、配置位置和运行环境。
- 能区分 ChatGPT 登录、API key 登录、access token、headless device auth 这些认证方式的适用场景。
- 能用 `codex doctor` 收集诊断信息，并知道哪些内容分享前需要脱敏。
- 能在升级、换机器、换账号或异常恢复后重新建立运行基线。

## 4. 适用场景

- 新机器首次安装 Codex。
- fork 或 clone 新仓库后准备开始学习。
- Codex 行为和文档不一致，需要先排除版本差异。
- 登录失效、浏览器登录失败、headless 机器无法打开浏览器。
- 本机 CLI、IDE extension、Codex app 使用同一账号但表现不同，需要确认版本和配置。
- 每次上游同步后，重新确认本地 CLI 是否仍能完成最小任务。

不适合用它直接判断模型能力、任务拆解质量或源码实现细节。它只解决“工具是否可用、环境是否清楚”的问题。

## 5. 功能边界

- 覆盖安装入口、版本确认、认证、启动 TUI、基础诊断、shell completion。
- 覆盖 CLI 本地使用，不展开 Codex app、IDE extension、cloud task 的完整工作流。
- 不负责解释 agent loop、工具调用或模型选择，只记录和启动相关的入口。
- 不把 `main` 分支作为学习材料保存位置；学习记录只进入 `00-study/codex-agent-learning`。

## 6. 使用约束

- 必须明确当前工作目录，优先在 Git 仓库中运行。
- 认证缓存、access token、`auth.json`、API key、环境变量都按敏感信息处理。
- API key 更适合程序化和自动化场景；ChatGPT 登录更适合普通本地交互和 ChatGPT 工作区能力。
- `codex exec` 中可以使用 `CODEX_API_KEY`，但不要把 key 设置成会被不可信构建脚本读取的长生命周期环境变量。
- headless 登录优先考虑 device auth；复制 `auth.json` 只适合可信机器，并且必须按密码级别保护。
- 在共享机器上使用后要考虑 `codex logout` 和凭据存储方式。

## 7. 用户入口

- `codex`：启动交互式 TUI。
- `codex "prompt"`：带初始 prompt 启动一次交互式任务并输出结果。
- `codex login`：进入登录流程。
- `codex login --device-auth`：使用 device code 登录，适合远程或 headless 环境。
- `codex login --with-access-token`：从 stdin 读取访问令牌。
- `codex logout`：移除本地认证状态。
- `codex doctor`：生成本地安装、配置、认证、运行时、Git、终端和线程清单诊断。
- `codex --version`：确认 CLI 版本。
- `codex completion bash|zsh|fish|power-shell|elvish`：生成 shell completion。
- `--cd`：指定 Codex 启动前进入的工作目录。
- `--strict-config`：发现当前版本不认识的配置项时直接报错，适合排查配置漂移。

## 8. 典型工作流

1. 进入目标仓库：`cd /path/to/repo`。
2. 检查分支和工作区：`git status --short --branch`。
3. 确认版本：`codex --version`。
4. 如果未登录或要切换账号，运行 `codex login`。
5. 运行 `codex doctor`，记录明显异常。
6. 在真实终端里用只读 prompt 启动最小交互任务：`codex "Summarize this repository in 5 bullets"`。
7. 在非交互 shell 里用 `exec` 验证一次性任务：`codex --ask-for-approval never exec --sandbox read-only --color never "Summarize this repository in 5 bullets"`。
8. 在 TUI 内用 `/status` 确认模型、权限、工作目录、上下文容量。
9. 如果只是初始化 shell 体验，再安装 completion。

## 9. 最佳实践

- 把版本、认证方式、当前目录、权限模式作为每次学习前的四个固定检查项。
- 第一次登录后立即做一次 `codex doctor`，保存你能理解的字段名称，不保存敏感值。
- 遇到异常先确认 `codex --version` 和 `codex doctor`，再怀疑 prompt 或模型。
- 在 headless 环境中先确认 browser callback 是否可用；不可用时用 device auth，不要临时乱改网络和安全设置。
- 复制日志给别人前，先搜索 `token`、`key`、`auth`、`secret`、邮箱、workspace ID 等敏感片段。
- 如果 CLI 和 app 行为不同，分别确认它们各自捆绑或调用的 Codex 版本。

## 10. 常见错误

- 只确认 `codex` 命令存在，就认为环境已经可用。
- 在非 Git 目录里运行修改任务，却没有理解 Codex 对 Git 仓库的安全假设。
- 把 `~/.codex/auth.json` 粘贴到 issue、聊天记录或日志里。
- 忽略 `CODEX_HOME`，导致自己改的是另一个配置目录。
- 登录 API key 后期待所有 ChatGPT workspace 或 cloud 能力都可用。
- 机器上有多个 Codex 安装来源，实际执行的不是自己刚升级的版本。

## 11. 风险和限制

- 认证成功不代表当前任务权限足够，sandbox、approval、network 仍需要单独确认。
- `codex doctor` 是诊断证据，不是自动修复工具。
- `codex login` 直接产生的登录日志和 auth cache 可能包含敏感信息。
- completion 只提升命令输入效率，不会改变 Codex 行为。
- 上游版本变化可能改变默认模型、配置键、命令 flags 或 feature maturity，需要定期回看 manual。

## 12. 验证方式

最小验证：

```bash
git status --short --branch
codex --version
codex doctor
codex --ask-for-approval never exec --sandbox read-only --color never "Summarize this repository in 5 bullets"
```

通过标准：

- `codex --version` 能正常输出。
- `codex doctor` 没有阻断性错误，或者错误已经被记录为待处理事项。
- 最小 `exec` prompt 能在目标仓库运行。
- 输出不要求完全正确，但必须能证明 Codex 读到了当前仓库上下文。

命令位置约束：

- 当前版本中 `--ask-for-approval` 是 `codex` 顶层参数，放在 `exec` 前面；`codex exec --ask-for-approval never ...` 会报 `unexpected argument`。
- `codex "prompt"` 属于交互式入口，在非 TTY shell 中可能直接失败并提示 `stdin is not a terminal`；非交互验证优先使用 `codex exec`。

## 13. 最小练习

在学习分支上完成一次启动基线记录：

1. 运行 `git status --short --branch`。
2. 运行 `codex --version`。
3. 运行 `codex doctor`，只记录诊断项类型，不记录敏感值。
4. 用 `codex exec` 启动一次只读解释任务。
5. 在 issue #1 记录：版本、认证方式类型、当前目录、doctor 是否有异常、下一步。

## 14. 进阶练习

- 在一个 headless shell 中尝试 device auth，记录和浏览器登录的差异。
- 设置临时 `CODEX_HOME=$(mktemp -d)` 启动 Codex，观察配置和认证状态如何隔离。
- 分别用 `--cd` 和手动 `cd` 启动，确认工作目录、Git 根目录和 `/status` 显示是否一致。
- 运行 `codex completion zsh` 或当前 shell 对应命令，理解 completion 不改变运行配置。

## 15. 实验设计

实验目标：证明“环境基线问题”和“agent 任务质量问题”可以分开排查。

变量：

- 目录：Git 仓库根目录、仓库子目录、非 Git 临时目录。
- 认证：当前登录状态、退出后状态、临时 `CODEX_HOME` 状态。
- 启动方式：`codex`、`codex "prompt"`、`codex --cd <path> "prompt"`。

记录字段：

- 命令。
- 退出状态。
- 是否能进入 TUI 或输出最终回答。
- `/status` 或 `doctor` 中最关键的差异。
- 是否涉及敏感信息，是否需要脱敏。

结论写回：

- 稳定结论写到本文件“当前结论”。
- 过程观察写到 issue #1。
- 如果发现命令行为和 manual 不一致，标记 `need-source-reading` 或检查版本。

## 16. 相关文档

- OpenAI Codex manual：CLI command reference。
- OpenAI Codex manual：Authentication and sessions。
- OpenAI Codex manual：Codex CLI features。
- OpenAI Codex manual：Troubleshooting。
- 本仓库：`README.md`。
- 本仓库：`docs/install.md`。
- 本仓库：`docs/authentication.md`。
- 本仓库：`docs/getting-started.md`。

## 17. 源码入口

- `codex-cli/bin/codex.js`：npm 包装层入口。
- `codex-rs/cli/src/main.rs`：CLI 子命令解析和启动入口。
- `codex-rs/cli/src/login.rs`：CLI 登录命令入口。
- `codex-rs/login/src/`：认证流程、token、device auth、storage。
- `codex-rs/cli/src/doctor.rs` 和 `codex-rs/cli/src/doctor/`：doctor 诊断入口和各类检查。
- `codex-rs/codex-home/src/`：Codex home 目录解析。
- `codex-rs/keyring-store/src/`：凭据存储相关实现。

## 18. 待解决问题

- `CODEX_HOME` 改变后，哪些状态会重新初始化，哪些仍来自系统或环境？
- ChatGPT 登录和 API key 登录在本学习目标下分别会限制哪些功能？
- 2026-07-10 的子 TUI `/status` 显示 `Agents.md: <none>`，但当前仓库根目录存在 `AGENTS.md`；需要在后续源码阅读时确认 nested Codex 的 guidance 发现条件和当前环境的差异。

## 19. 当前结论

安装、登录、启动的目标不是“能打开 Codex”这么简单，而是建立可复现的运行基线。阶段 1 每次学习前都应先确认分支、版本、认证和诊断状态；一旦遇到异常，先回到这个基线排查，再进入具体功能。

2026-07-08 基线记录：

- 当前分支是 `00-study/codex-agent-learning`，工作区开始时干净。
- `codex --version` 输出 `codex-cli 0.142.5`。
- 当前 CLI 来自 npm 安装的发布包；`codex doctor` 显示 runtime/install method 为 npm，安装一致。
- `codex doctor` 结果为 `16 ok · 1 idle · 2 notes · 0 warn · 1 fail`。
- 这个 `fail` 来自当前非交互 shell 的终端环境：`TERM=dumb`，颜色和光标控制不可用；它不是认证失败，也不是模型不可用。
- `doctor` 提示有新版本 `0.143.0` 可用；在升级前，学习记录应继续注明当前基线版本，避免把版本差异误判为功能差异。
- 当前认证状态已配置，存储模式为 File，stored auth mode 为 ChatGPT，未保存 API key，已保存 ChatGPT tokens。记录 issue 或文档时只写认证类型，不写 token、账号标识或 auth 文件内容。
- 当前默认模型为 `gpt-5.5`，provider 为 `openai`，MCP servers 为 `0`。
- 当前配置文件为 `~/.codex/config.toml`，解析正常。
- 当前 sandbox/approval 基线来自 `doctor`：restricted filesystem + restricted network，approval policy 为 `OnRequest`。
- `codex "Summarize this repository in 5 bullets"` 在非 TTY shell 中失败，错误是 `stdin is not a terminal`；这个命令仍适合真实终端/TUI，不适合作为自动化基线。
- `codex --ask-for-approval never exec --sandbox read-only --color never "Summarize this repository in 5 bullets"` 在非交互环境中跑通，显示 workdir、model、provider、approval、sandbox 和 session id，并能读取仓库文件后生成五点摘要。
- 当前版本的参数位置要特别注意：`--ask-for-approval` 必须放在 `exec` 前面；把它放在 `exec` 后会被 `codex exec` 解析为未知参数。

本功能的阶段 1 验收为：`version`、`doctor`、非交互 `exec` 和真实 TUI `/status` 均已跑通，功能地图状态为 `Done`。

2026-07-10 TUI 补充实验：

- 当前 `codex --version` 为 `0.144.1`，所以 2026-07-08 的 `0.142.5` 基线只适用于当时记录；后续结论必须注明版本。
- 通过本地 `codex-tui-observer` MCP 在 PTY 中启动子 Codex，并固定使用 `--ask-for-approval never --sandbox read-only`。这能观察真实 TUI，同时避免实验修改工作区。
- `/status` 正常展示模型 `gpt-5.6-terra`（reasoning `high`）、工作目录 `~/codex`、权限 `Read Only (never)`、协作模式 `Default` 和上下文容量；账号字段已在记录中脱敏。
- 此次实验补齐了交互式状态页验证，因此本功能达到阶段 1 的既定验收标准并标记为 `Done`。
