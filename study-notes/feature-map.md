# Codex CLI 功能地图

这个文件保存整理后的功能学习卡片。GitHub Issues 用作过程看板和手机端记录，本文件只沉淀已经梳理过的结构化理解。

## 阅读顺序

1. 先看功能索引，选择一个 `Todo` 或 `In Progress` 功能。
2. 打开对应 GitHub issue，查看过程记录和最新问题。
3. 本地学习和实验后，把稳定结论整理回本文件。
4. 完成后把卡片状态改为 `Done`，并关闭对应 issue。

## 状态说明

- `Todo`：还没开始。
- `In Progress`：正在学习。
- `Need experiment`：需要最小实验验证。
- `Need source reading`：需要进入源码确认实现。
- `Done`：已形成可复用结论。

## 功能索引

| 功能 | 状态 | Issue |
| --- | --- | --- |
| 安装、登录、启动 | Todo | [#1](https://github.com/liqiangcc/codex/issues/1) |
| 交互式 coding agent | Todo | [#2](https://github.com/liqiangcc/codex/issues/2) |
| 非交互执行 | Todo | [#3](https://github.com/liqiangcc/codex/issues/3) |
| 任务上下文 | Todo | [#4](https://github.com/liqiangcc/codex/issues/4) |
| 权限和安全 | Todo | [#5](https://github.com/liqiangcc/codex/issues/5) |
| 项目约定 | Todo | [#6](https://github.com/liqiangcc/codex/issues/6) |
| 配置系统 | Todo | [#7](https://github.com/liqiangcc/codex/issues/7) |
| 工具和扩展 | Todo | [#8](https://github.com/liqiangcc/codex/issues/8) |
| 调试和诊断 | Todo | [#9](https://github.com/liqiangcc/codex/issues/9) |
| Git 和协作 | Todo | [#10](https://github.com/liqiangcc/codex/issues/10) |

## 阶段 1 功能覆盖清单

阶段 1 不为每个细功能单独开一级卡片，而是把它们映射到上面的 10 张功能卡片。这样学习路线保持稳定，同时能确认 Codex CLI 的主要使用面没有漏掉。

| Codex 细功能 | 所属功能卡片 | 阶段 1 深度 | 备注 |
| --- | --- | --- | --- |
| 安装 CLI、版本确认、启动 TUI | 安装、登录、启动 | 深入 | 建立所有学习任务的运行基线。 |
| `codex login`、`logout`、认证方式 | 安装、登录、启动 | 深入 | 重点理解登录状态、凭据和故障排查。 |
| `codex doctor`、环境诊断 | 安装、登录、启动；调试和诊断 | 深入 | 既是启动检查，也是异常排查入口。 |
| 交互式 TUI、composer、初始 prompt | 交互式 coding agent | 深入 | 第一阶段核心能力。 |
| 让 Codex 读代码、改代码、跑命令、跑测试 | 交互式 coding agent | 深入 | 每次学习都要落到可验证任务。 |
| `/plan`、先计划再执行 | 交互式 coding agent | 深入 | 用于控制复杂任务风险。 |
| `/diff`、查看本地变更 | Git 和协作 | 深入 | 每次写文件后都要检查。 |
| `/review`、本地代码 review | Git 和协作 | 基础 | 先掌握使用场景，后续再读实现。 |
| `codex exec`、一次性 prompt | 非交互执行 | 深入 | 学会和交互式模式做取舍。 |
| `codex exec --json`、JSONL 输出 | 非交互执行 | 基础 | 先理解自动化价值，不急着做复杂脚本。 |
| `codex exec resume` | 非交互执行；任务上下文 | 基础 | 关注历史上下文如何延续。 |
| `resume`、`/resume`、继续会话 | 任务上下文 | 基础 | 学会恢复上下文，避免重复说明。 |
| `/new`、`/clear`、`/compact` | 任务上下文 | 基础 | 重点理解什么时候重置或压缩上下文。 |
| `/fork`、`/side`、分支对话 | 任务上下文 | 浅用 | 作为探索替代方案的工具，暂不深入实现。 |
| Prompt 写法、验收标准、范围控制 | 任务上下文 | 深入 | 直接决定 Codex 输出质量。 |
| 文件路径、错误日志、图片输入 | 任务上下文 | 基础 | 图片输入先作为可选上下文能力。 |
| Web search、`--search`、`web_search` 配置 | 任务上下文；权限和安全 | 基础 | 重点理解新鲜度、注入风险和网络边界。 |
| sandbox、approval、`/permissions` | 权限和安全 | 深入 | 第一阶段必须掌握。 |
| `--yolo`、Full Access | 权限和安全 | 基础 | 只理解风险，不作为默认使用方式。 |
| `execpolicy` | 权限和安全 | 基础 | 先知道它解决命令策略问题，后续再源码深入。 |
| network 权限和外部访问 | 权限和安全 | 基础 | 和 web search、MCP、命令执行一起理解。 |
| `AGENTS.md`、`/init` | 项目约定 | 深入 | 适合沉淀仓库长期规则。 |
| custom prompts、rules | 项目约定 | 基础 | 先区分一次性 prompt 和长期规则。 |
| `config.toml`、profiles、`-c key=value` | 配置系统 | 深入 | 重点理解配置优先级和适用范围。 |
| model、reasoning、`/model`、`review_model` | 配置系统 | 基础 | 学会选择和检查，不做模型评测。 |
| feature flags、`codex features`、`/experimental` | 配置系统 | 基础 | 只启用确实需要验证的功能。 |
| TUI 主题、keymap、vim、statusline、title | 交互式 coding agent | 浅用 | 效率功能，阶段 1 不深入。 |
| shell completions | 安装、登录、启动 | 浅用 | 提升日常使用效率即可。 |
| MCP、`codex mcp`、`/mcp` | 工具和扩展 | 基础 | 重点理解外部工具接入边界。 |
| skills、`/skills` | 工具和扩展 | 基础 | 用于复用任务工作流。 |
| hooks、`/hooks` | 工具和扩展；权限和安全 | 基础 | 重点关注信任和生命周期。 |
| plugins、`codex plugin`、`/plugins` | 工具和扩展 | 基础 | 先理解分发和安装，不做插件开发。 |
| apps/connectors、`/apps` | 工具和扩展 | 浅用 | 当前阶段只了解概念和授权边界。 |
| subagents、`/agent` | 工具和扩展；任务上下文 | 可选 | 阶段 1 不作为必学项，后续作为高级 agent 主题。 |
| `/status`、`/usage`、`/debug-config` | 调试和诊断 | 深入 | 用于确认模型、权限、上下文和配置。 |
| `codex debug models` | 调试和诊断；配置系统 | 基础 | 用于理解模型目录和排查模型问题。 |
| `/ps`、`/stop`、后台命令状态 | 调试和诊断 | 基础 | 用于处理长时间运行或卡住的命令。 |
| `/feedback` | 调试和诊断 | 浅用 | 只在需要反馈问题时使用。 |
| Git dirty state、diff、commit、push | Git 和协作 | 深入 | 学习分支的日常闭环。 |
| GitHub Issues 学习看板 | Git 和协作 | 深入 | 手机端过程记录和状态跟踪。 |
| PR、review comments、上游贡献 | Git 和协作 | 基础 | 阶段 1 只掌握概念，不向上游发 PR。 |
| Codex IDE extension | 暂不纳入阶段 1 | 暂不深入 | 后续可作为单独阶段。 |
| Codex app、Codex cloud | 暂不纳入阶段 1 | 暂不深入 | 先把 CLI 学扎实。 |
| app-server、SDK、GitHub Action、remote-control | 暂不纳入阶段 1 | 暂不深入 | 属于程序化接口或开发者扩展阶段。 |

如果某个细功能无法自然归入这张表中的任何卡片，再考虑新增一级功能卡片。

## 卡片模板

```markdown
## 功能：名称

### 摘要

- 状态：Todo / In Progress / Need experiment / Need source reading / Done
- Issue：
- 一句话说明：
- 本轮目标：
- 当前结论：
- 下一步：

### 1. 这个功能是什么

用简单语言说明功能本身，不进入实现细节。

### 2. 为什么需要它

说明背景、解决的问题、没有它会怎样。

### 3. 它要达成什么

写清功能目标和成功标准。

### 4. 适合什么场景

列出适合使用和不适合使用的场景。

### 5. 边界和约束

- 功能边界：
- 使用约束：
- 容易混淆：

### 6. 怎么使用

- 用户入口：
- 典型工作流：

### 7. 怎么用得好

记录最佳实践：如何给上下文、控制范围、控制权限、要求验证。

### 8. 风险和限制

记录误用点、失败场景、安全风险、上下文限制。

### 9. 怎么验证

说明如何确认结果可信：命令、测试、diff、日志、复现实验。

### 10. 文档和源码入口

- 相关文档：
- 源码入口：
- 相关实验：

### 11. 待解决问题

记录下一步要验证或读源码的问题。
```

## 功能：安装、登录、启动

### 摘要

- 状态：Todo
- Issue：[#1](https://github.com/liqiangcc/codex/issues/1)
- 一句话说明：让 Codex CLI 在本机可运行、可认证、可进入目标仓库工作。
- 本轮目标：掌握安装、登录、版本确认、启动和基础故障排查。
- 当前结论：优先从官方安装、`codex login`、`codex doctor` 和 `codex --version` 建立使用基线。
- 下一步：验证本机 Codex CLI 版本、认证方式和 doctor 输出。

### 1. 这个功能是什么

这是使用 Codex 的入口能力，包括安装 CLI、完成认证、确认版本、进入目标目录并启动交互会话。

### 2. 为什么需要它

没有稳定的安装和登录基线，后续使用、配置、权限、MCP、skills 和问题排查都没有可靠前提。

### 3. 它要达成什么

成功标准是能在目标仓库中启动 Codex，并确认当前账号、模型能力、配置和运行环境没有明显问题。

### 4. 适合什么场景

适合新机器初始化、升级后检查、认证失效排查、切换工作目录前确认环境。不适合直接用来判断 agent 行为质量。

### 5. 边界和约束

- 功能边界：只覆盖 CLI 可用性和认证状态，不覆盖具体开发任务。
- 使用约束：需要有效的 ChatGPT 登录、API key 或支持的认证方式。
- 容易混淆：安装成功不代表当前仓库权限、sandbox 或网络策略都符合任务要求。

### 6. 怎么使用

- 用户入口：`codex`、`codex login`、`codex logout`、`codex doctor`、`codex --version`。
- 典型工作流：安装 CLI，登录，进入仓库，运行 doctor，启动 `codex`。

### 7. 怎么用得好

每次换机器、升级版本或遇到异常时先确认版本和 doctor 报告，再排查配置或任务本身。

### 8. 风险和限制

认证信息属于敏感状态，日志和截图需要避免泄露 token、账号或环境变量。

### 9. 怎么验证

运行 `codex --version`、`codex doctor`，并在目标仓库中启动一次简单解释任务。

### 10. 文档和源码入口

- 相关文档：`README.md`、`docs/install.md`、`docs/authentication.md`、Codex manual 的 CLI command reference。
- 源码入口：`codex-cli/bin/codex.js`、`codex-rs/cli/src/main.rs`、`codex-rs/cli/src/login.rs`、`codex-rs/cli/src/doctor.rs`。
- 相关实验：TBD。

### 11. 待解决问题

- 当前本机安装的是源码构建版本还是发布版？
- `codex doctor` 的输出哪些字段最适合作为使用前检查项？

## 功能：交互式 coding agent

### 摘要

- 状态：Todo
- Issue：[#2](https://github.com/liqiangcc/codex/issues/2)
- 一句话说明：在 TUI 中和 Codex 协作读代码、改代码、运行命令、验证结果。
- 本轮目标：掌握一次真实开发任务从 prompt 到 diff 再到验证的完整流程。
- 当前结论：这是第一阶段最核心的功能，应优先学习。
- 下一步：选择一个小任务，让 Codex 先计划、再执行、再 review diff。

### 1. 这个功能是什么

交互式 coding agent 是 Codex CLI 的主要使用方式。用户在终端里提出任务，Codex 读取仓库、修改文件、运行命令并持续解释行动。

### 2. 为什么需要它

真实开发任务通常需要多轮澄清、检查 diff、运行测试和修正结果，交互式模式比一次性命令更适合控制过程。

### 3. 它要达成什么

成功标准是 Codex 能在明确范围内完成改动，并留下可检查的 diff、测试结果和残留风险说明。

### 4. 适合什么场景

适合代码解释、小功能实现、bug fix、测试补充、重构前分析和本地 review。不适合高风险批量操作或需求边界不清的大改造。

### 5. 边界和约束

- 功能边界：覆盖本地仓库协作，不等同于 cloud task 或 IDE extension。
- 使用约束：任务范围要清楚，工作区状态要可控。
- 容易混淆：聊天回答不是完成任务，完成任务必须看 diff 和验证结果。

### 6. 怎么使用

- 用户入口：`codex`、`codex "prompt"`、TUI prompt、`/plan`、`/diff`、`/review`、`/status`。
- 典型工作流：说明目标，限制范围，让 Codex 读代码，确认计划，执行，检查 diff，跑测试，复盘。

### 7. 怎么用得好

给出目标、范围、验收标准和禁止事项。复杂任务先要求计划，修改前确认文件范围，修改后要求测试和 diff 总结。

### 8. 风险和限制

容易因为 prompt 太大、边界不清或未检查 dirty state 造成无关改动。不能把 Codex 输出直接当作事实，必须验证。

### 9. 怎么验证

使用 `git diff`、相关测试命令、lint/build、`/review` 或人工检查关键路径。

### 10. 文档和源码入口

- 相关文档：`docs/getting-started.md`、Codex manual 的 Codex CLI features、Execution Model and Workflows。
- 源码入口：`codex-rs/cli/`、`codex-rs/core/`、`codex-rs/tui/`。
- 相关实验：TBD。

### 11. 待解决问题

- 一条 TUI 用户消息如何进入 core？
- Codex 在什么时机决定读取文件、编辑文件或运行命令？

## 功能：非交互执行

### 摘要

- 状态：Todo
- Issue：[#3](https://github.com/liqiangcc/codex/issues/3)
- 一句话说明：用 `codex exec` 在脚本或一次性命令中运行 Codex。
- 本轮目标：理解非交互模式适合什么自动化任务，以及如何控制输出和失败。
- 当前结论：它适合明确、边界小、可自动验证的任务。
- 下一步：用一个只读解释任务和一个小型验证任务对比交互式模式。

### 1. 这个功能是什么

非交互执行让 Codex 在没有 TUI 多轮交互的情况下完成一个明确 prompt，并把结果输出到终端或结构化流。

### 2. 为什么需要它

有些任务适合脚本化，例如解释代码、生成报告、检查 CI 失败、批量整理文本或执行固定工作流。

### 3. 它要达成什么

成功标准是输入明确、输出可消费、失败可检测，并能在脚本或 CI 中判断结果。

### 4. 适合什么场景

适合低歧义、可复现、可验证的任务。不适合需要频繁澄清或人工判断的大范围修改。

### 5. 边界和约束

- 功能边界：一次任务流，不是持续协作会话。
- 使用约束：prompt 必须包含足够上下文和验收标准。
- 容易混淆：非交互不等于无人监督，尤其是涉及写文件和运行命令时。

### 6. 怎么使用

- 用户入口：`codex exec`、`codex exec resume`、`--json`、`--cd`、`--sandbox`、`--ask-for-approval`。
- 典型工作流：写清 prompt，固定目录和权限，执行，检查退出码、输出和 diff。

### 7. 怎么用得好

把任务拆小，要求 Codex 输出结果摘要和验证命令。自动化场景优先选择只读或低风险任务。

### 8. 风险和限制

缺少交互澄清，容易在需求不清时做错方向。脚本中如果权限过宽，风险会被放大。

### 9. 怎么验证

检查退出码、JSONL 输出、工作区 diff、测试结果和日志。

### 10. 文档和源码入口

- 相关文档：`docs/exec.md`、Codex manual 的 Non-interactive mode、CLI command reference。
- 源码入口：`codex-rs/cli/src/main.rs`、`codex-rs/exec/`、`codex-rs/core/`。
- 相关实验：TBD。

### 11. 待解决问题

- `codex exec` 和 TUI 在上下文、审批和输出上有哪些差异？
- JSONL 输出里哪些事件最适合自动化消费？

## 功能：任务上下文

### 摘要

- 状态：Todo
- Issue：[#4](https://github.com/liqiangcc/codex/issues/4)
- 一句话说明：管理 Codex 做任务时能看到和应该关注的上下文。
- 本轮目标：理解 prompt、文件、Git 状态、历史对话、图片和外部资料如何影响结果。
- 当前结论：上下文质量直接决定 Codex 输出质量。
- 下一步：比较“只给目标”和“给目标加路径、日志、验收标准”的效果。

### 1. 这个功能是什么

任务上下文是 Codex 用来理解任务的一切输入，包括当前目录、仓库文件、用户 prompt、历史消息、Git 状态、图片、web search 和 MCP 工具结果。

### 2. 为什么需要它

Codex 不是单纯执行命令，它需要根据上下文判断目标、边界和风险。上下文不足会导致误判，过多会导致噪声。

### 3. 它要达成什么

成功标准是 Codex 能获得足够信息，同时任务范围仍然清晰可控。

### 4. 适合什么场景

适合所有 Codex 任务，尤其是跨文件修改、bug 排查、测试失败分析和架构理解。

### 5. 边界和约束

- 功能边界：关注信息输入和上下文组织，不等同于模型能力本身。
- 使用约束：敏感信息、私有日志和大段无关内容需要控制。
- 容易混淆：给更多上下文不一定更好，关键是给相关上下文。

### 6. 怎么使用

- 用户入口：prompt、路径、错误日志、`--image`、`--add-dir`、`/mention`、`/compact`、`/status`。
- 典型工作流：说明目标，给关键路径或日志，要求 Codex 自己搜索相关代码，再让它确认理解。

### 7. 怎么用得好

优先给目标、范围、关键文件、错误输出和验收标准。长任务中定期压缩和复盘，避免上下文漂移。

### 8. 风险和限制

上下文可能过期、过长或包含误导信息。外部搜索和网页内容要当作不可信输入处理。

### 9. 怎么验证

让 Codex 复述任务理解、列出将修改的文件和验证方式，再开始执行。

### 10. 文档和源码入口

- 相关文档：Codex manual 的 Prompting、Best practices、Slash commands、Web search。
- 源码入口：`codex-rs/core/`、`codex-rs/context-fragments/`、`codex-rs/rollout-trace/`。
- 相关实验：TBD。

### 11. 待解决问题

- Codex 如何选择自动读取哪些文件？
- `/compact` 具体保留哪些信息？

## 功能：权限和安全

### 摘要

- 状态：Todo
- Issue：[#5](https://github.com/liqiangcc/codex/issues/5)
- 一句话说明：控制 Codex 可以读写什么、运行什么、什么时候必须确认。
- 本轮目标：理解 sandbox、approval、network 和 exec policy 的职责边界。
- 当前结论：权限安全是用好 Codex 的核心前提。
- 下一步：设计一个读写当前仓库、访问仓库外目录、访问网络的对照实验。

### 1. 这个功能是什么

权限和安全能力用于限制 Codex 的文件访问、命令执行、网络访问和高风险操作确认流程。

### 2. 为什么需要它

Codex 能读写文件和运行命令，如果没有边界，可能误改无关文件、泄露数据或执行高风险命令。

### 3. 它要达成什么

成功标准是 Codex 能完成必要操作，同时高风险动作被阻止、提示或需要用户确认。

### 4. 适合什么场景

适合所有会修改文件、运行命令或联网的任务。只读解释任务也应知道当前权限状态。

### 5. 边界和约束

- 功能边界：sandbox 控制环境边界，approval 控制确认流程，exec policy 控制命令策略。
- 使用约束：不要在不可信仓库中使用 full access 或绕过审批。
- 容易混淆：approval 放行不代表命令一定安全，只代表用户允许尝试。

### 6. 怎么使用

- 用户入口：`--sandbox`、`--ask-for-approval`、`--yolo`、`/permissions`、`docs/sandbox.md` 中的配置。
- 典型工作流：默认 workspace-write，遇到外部目录或网络再明确授权。

### 7. 怎么用得好

保守起步，只给当前仓库写权限。高风险命令先要求 Codex 解释目的、影响范围和回滚方式。

### 8. 风险和限制

权限过宽会放大误操作风险。权限过窄会导致任务失败或产生不完整结论。

### 9. 怎么验证

使用 `/status` 查看当前权限，运行受控实验观察哪些操作会被允许、阻止或请求确认。

### 10. 文档和源码入口

- 相关文档：`docs/sandbox.md`、`docs/execpolicy.md`、Codex manual 的 Agent approvals & security、Sandbox、Permissions。
- 源码入口：`codex-rs/execpolicy/`、`codex-rs/exec/`、`codex-rs/bwrap/`、`codex-rs/core/`。
- 相关实验：TBD。

### 11. 待解决问题

- sandbox、approval policy、exec policy 的执行顺序是什么？
- 网络访问在不同权限模式下如何变化？

## 功能：项目约定

### 摘要

- 状态：Todo
- Issue：[#6](https://github.com/liqiangcc/codex/issues/6)
- 一句话说明：用 `AGENTS.md` 等机制把仓库长期规则交给 Codex。
- 本轮目标：理解哪些规则应该写进项目约定，哪些只适合写在单次 prompt。
- 当前结论：项目约定适合稳定、重复、和仓库相关的规则。
- 下一步：阅读本仓库 `AGENTS.md` 并总结规则类型。

### 1. 这个功能是什么

项目约定是 Codex 在仓库中工作时应长期遵守的说明，例如构建命令、测试方式、代码风格、目录职责和审核要求。

### 2. 为什么需要它

如果每次都在 prompt 中重复仓库规则，容易遗漏且不稳定。项目约定让 Codex 更接近团队成员的默认工作方式。

### 3. 它要达成什么

成功标准是 Codex 能自动遵守仓库规则，并在修改后执行合适的验证。

### 4. 适合什么场景

适合长期仓库规范、固定验证命令、子目录约束、贡献要求。不适合一次性需求或临时偏好。

### 5. 边界和约束

- 功能边界：`AGENTS.md` 管仓库内行为，不替代全局配置或任务 prompt。
- 使用约束：规则应短、明确、可执行。
- 容易混淆：项目约定不是文档大全，不应塞入所有背景知识。

### 6. 怎么使用

- 用户入口：`AGENTS.md`、`/init`、prompt 中引用项目规则。
- 典型工作流：为仓库写规则，Codex 执行任务时读取并遵守，修改后按规则验证。

### 7. 怎么用得好

写清构建、测试、风格、禁止事项和 review 标准。规则要能被执行和检查。

### 8. 风险和限制

规则太长会稀释重点。过期规则会误导 Codex。冲突规则会造成行为不稳定。

### 9. 怎么验证

设计一个任务，看 Codex 是否主动遵守 `AGENTS.md` 中的验证命令和边界。

### 10. 文档和源码入口

- 相关文档：`AGENTS.md`、`docs/agents_md.md`、Codex manual 的 Custom instructions with AGENTS.md。
- 源码入口：`codex-rs/core/`、`codex-rs/project-doc/`。
- 相关实验：TBD。

### 11. 待解决问题

- 嵌套 `AGENTS.md` 的优先级如何生效？
- Codex 如何把项目约定注入模型上下文？

## 功能：配置系统

### 摘要

- 状态：Todo
- Issue：[#7](https://github.com/liqiangcc/codex/issues/7)
- 一句话说明：用配置文件和命令行参数控制 Codex 默认行为。
- 本轮目标：理解 config、profiles、features、model、review_model、web_search 和权限配置的优先级。
- 当前结论：配置适合稳定默认值，prompt 适合一次性任务要求。
- 下一步：梳理 `docs/config.md` 和当前 config 示例。

### 1. 这个功能是什么

配置系统让用户持久化 Codex 的模型、权限、功能开关、MCP、hooks、TUI 和其他默认行为。

### 2. 为什么需要它

重复使用的偏好不应每次写进 prompt。配置能让 Codex 在不同仓库和任务中保持稳定默认行为。

### 3. 它要达成什么

成功标准是用户知道某个行为应该放在全局配置、项目配置、profile、命令行参数还是单次 prompt。

### 4. 适合什么场景

适合模型默认值、权限默认值、MCP、hooks、features、TUI 偏好和 review model 设置。

### 5. 边界和约束

- 功能边界：配置控制运行时默认值，不表达具体任务目标。
- 使用约束：配置层级和命令行覆盖需要明确。
- 容易混淆：`AGENTS.md` 是仓库指令，`config.toml` 是运行设置。

### 6. 怎么使用

- 用户入口：`~/.codex/config.toml`、profiles、`-c key=value`、`--profile`、`codex features`。
- 典型工作流：先用命令行临时覆盖，确认后再写入持久配置。

### 7. 怎么用得好

只把稳定偏好写配置。实验性设置先用一次性参数，避免污染长期默认行为。

### 8. 风险和限制

配置层级不清会导致行为和预期不一致。过度配置会让问题排查变难。

### 9. 怎么验证

用 `/status`、`/debug-config`、`codex features list` 和 `codex doctor` 检查实际生效配置。

### 10. 文档和源码入口

- 相关文档：`docs/config.md`、`docs/example-config.md`、Codex manual 的 Configuration Reference、Config basics、Advanced Configuration。
- 源码入口：`codex-rs/config/`、`codex-rs/cli/src/main.rs`。
- 相关实验：TBD。

### 11. 待解决问题

- 命令行 `-c`、profile、全局 config 的具体优先级如何实现？
- 哪些设置适合写入本仓库学习环境？

## 功能：工具和扩展

### 摘要

- 状态：Todo
- Issue：[#8](https://github.com/liqiangcc/codex/issues/8)
- 一句话说明：通过 MCP、skills、hooks、plugins 扩展 Codex 能力。
- 本轮目标：区分这几类扩展各自解决什么问题。
- 当前结论：MCP 连接外部工具，skills 固化工作流，hooks 管生命周期，plugins 打包能力。
- 下一步：为每种扩展找一个最小使用场景。

### 1. 这个功能是什么

工具和扩展让 Codex 能调用外部系统、复用任务流程、执行生命周期检查，或安装成套能力。

### 2. 为什么需要它

内置能力不可能覆盖所有私有系统、团队流程和专业任务。扩展机制让 Codex 能接入具体工作环境。

### 3. 它要达成什么

成功标准是用户能判断某个需求应该用 MCP、skill、hook、plugin 还是普通 prompt。

### 4. 适合什么场景

适合接入私有数据、标准化重复任务、执行安全检查、封装团队工具。不适合临时的一句话需求。

### 5. 边界和约束

- 功能边界：扩展机制不同，生命周期和安全模型不同。
- 使用约束：外部工具权限、认证和信任边界必须清楚。
- 容易混淆：skill 是工作流知识，MCP 是工具接口，plugin 是分发包。

### 6. 怎么使用

- 用户入口：`codex mcp`、`/mcp`、`/skills`、`/hooks`、`/plugins`、`codex plugin`。
- 典型工作流：先用 prompt 验证需求，再把稳定流程固化成 skill 或 plugin。

### 7. 怎么用得好

先明确扩展要解决的问题和权限边界。不要为了简单任务引入过重扩展。

### 8. 风险和限制

扩展会扩大 Codex 能访问的系统和数据。hook 和 plugin 需要特别注意信任和可审计性。

### 9. 怎么验证

列出启用的工具，做一个最小调用，检查输出、权限、日志和失败路径。

### 10. 文档和源码入口

- 相关文档：`docs/skills.md`、Codex manual 的 Agent Skills、Model Context Protocol、Hooks、Plugins。
- 源码入口：`codex-rs/codex-mcp/`、`codex-rs/core-skills/`、`codex-rs/hooks/`、`codex-rs/core-plugins/`。
- 相关实验：TBD。

### 11. 待解决问题

- 什么时候应该写 skill，什么时候应该写 plugin？
- MCP 工具如何进入模型可调用工具列表？

## 功能：调试和诊断

### 摘要

- 状态：Todo
- Issue：[#9](https://github.com/liqiangcc/codex/issues/9)
- 一句话说明：当 Codex 行为异常时，用诊断命令、日志和状态信息定位问题。
- 本轮目标：掌握 doctor、debug、status、usage、配置诊断和失败复盘。
- 当前结论：诊断能力应该在学习早期掌握，避免遇到问题只能猜。
- 下一步：跑一次 doctor，记录哪些字段值得长期关注。

### 1. 这个功能是什么

调试和诊断能力帮助用户理解 Codex 当前安装、认证、配置、模型、权限、终端、线程和运行状态。

### 2. 为什么需要它

Codex 失败可能来自认证、配置、权限、网络、模型、仓库状态或工具调用。没有诊断信息，很难定位。

### 3. 它要达成什么

成功标准是遇到异常时能快速收集证据，并判断下一步该检查配置、权限、网络、仓库还是 prompt。

### 4. 适合什么场景

适合启动失败、认证问题、命令失败、权限异常、模型不可用、上下文混乱和输出不符合预期。

### 5. 边界和约束

- 功能边界：诊断提供证据，不自动修复所有问题。
- 使用约束：诊断报告可能包含环境信息，分享前需要检查。
- 容易混淆：模型回答质量问题不一定是系统故障。

### 6. 怎么使用

- 用户入口：`codex doctor`、`codex debug models`、`/status`、`/usage`、`/debug-config`、`/ps`、`/stop`。
- 典型工作流：复现问题，收集状态，缩小原因，修正配置或任务，重新验证。

### 7. 怎么用得好

先记录现象和复现步骤，再看诊断输出。不要只截屏，尽量保存可搜索文本。

### 8. 风险和限制

诊断输出需要脱敏。某些问题来自外部服务或权限，不一定能通过本地命令解决。

### 9. 怎么验证

修正后重复最小复现步骤，确认失败不再出现，并记录原因。

### 10. 文档和源码入口

- 相关文档：Codex manual 的 Troubleshooting、CLI command reference。
- 源码入口：`codex-rs/cli/src/doctor/`、`codex-rs/cli/tests/debug_models.rs`。
- 相关实验：TBD。

### 11. 待解决问题

- `codex doctor --json` 在当前版本输出哪些诊断项？
- 哪些字段适合放入 issue 或 bug report？

## 功能：Git 和协作

### 摘要

- 状态：Todo
- Issue：[#10](https://github.com/liqiangcc/codex/issues/10)
- 一句话说明：让 Codex 在 Git 工作流中安全地产生、检查、提交和同步变更。
- 本轮目标：掌握 dirty state、diff review、commit、PR、上游同步和 issue 闭环。
- 当前结论：Git 是验证 Codex 工作结果的核心外壳。
- 下一步：用学习分支流程验证一次从文档修改到提交推送的闭环。

### 1. 这个功能是什么

Git 和协作能力覆盖 Codex 如何在仓库中识别状态、避免覆盖用户改动、展示 diff、运行 review、提交推送和关联 issue。

### 2. 为什么需要它

Codex 的产出最终要进入版本控制。没有 Git 约束，很难确认它改了什么、是否安全、如何回滚。

### 3. 它要达成什么

成功标准是每次变更都有清晰 diff、验证结果、提交记录和必要的 issue 关联。

### 4. 适合什么场景

适合任何会修改文件的任务，尤其是长期学习分支、bug fix、测试补充和准备 PR。

### 5. 边界和约束

- 功能边界：本阶段以个人 fork 和学习分支为主，不向上游直接提交 PR。
- 使用约束：不能覆盖用户未确认的改动，不能把学习内容放入 `main`。
- 容易混淆：fork 同步上游和向上游贡献是两条不同流程。

### 6. 怎么使用

- 用户入口：`git status`、`git diff`、`git commit`、`git push`、`gh issue`、`gh pr`、`/diff`、`/review`。
- 典型工作流：确认分支，修改，检查 diff，验证，提交，推送，更新 issue。

### 7. 怎么用得好

小步提交，提交消息说明学习目标或实验结论。每次同步上游前先确认工作区干净。

### 8. 风险和限制

rebase 后需要 `--force-with-lease`，必须确认当前分支。不要在 `main` 上写学习材料。

### 9. 怎么验证

检查 `git status --short --branch`、`git log --oneline`、远端分支和 issue 状态。

### 10. 文档和源码入口

- 相关文档：`docs/contributing.md`、Codex manual 的 Review、Worktrees、Codex code review in GitHub。
- 源码入口：`codex-rs/core/` 中 Git 状态相关逻辑、`codex-rs/cli/` 中 review 入口。
- 相关实验：TBD。

### 11. 待解决问题

- Codex 如何判断哪些 dirty changes 是用户已有改动？
- `/review` 和普通 prompt review 的执行路径有什么不同？
