# 功能：交互式 coding agent

## 1. 一句话说明

在终端 TUI 中和 Codex 多轮协作，让它读代码、提出计划、修改文件、运行命令，并通过 diff 和验证结果完成真实开发任务。

## 2. 背景和动机

Codex CLI 的核心价值不是“回答代码问题”，而是作为本地仓库中的 coding agent 参与开发循环。真实任务往往需要先理解代码，再提出方案，再小步修改，再运行测试，再根据失败结果修正。交互式 TUI 正是为了支持这个循环：用户可以随时补充上下文、调整权限、查看 diff、切换模型、压缩上下文、运行 review 或恢复旧会话。

如果只把 Codex 当聊天工具，容易停留在“它说得对不对”。第一阶段的目标是把它当工程协作者：它必须产出可检查的 diff、可复现的命令和明确的残留风险。

## 3. 用户目标

- 能把模糊需求改写成 Codex 可以执行的任务。
- 能判断何时先要求计划，何时可以直接实现。
- 能在修改前控制范围，在修改后检查 diff 和测试结果。
- 能用 slash commands 调整会话状态，而不是重启解决所有问题。
- 能通过多轮反馈把 Codex 拉回目标，不让任务漂移。

## 4. 适用场景

- 解释一个陌生代码路径。
- 实现一个小功能或文档改动。
- 修复一个有日志或复现步骤的 bug。
- 添加或更新测试。
- 根据失败测试定位问题。
- 对本地变更做一次 review。
- 把一次学习过程沉淀为文档、实验或 issue。

不适合：

- 需求边界完全不清的大范围改造。
- 不允许本地验证的高风险生产变更。
- 需要访问未授权外部系统的任务。
- 用户不准备检查 diff 就让 Codex 自主提交的任务。

## 5. 功能边界

- 这里关注 CLI TUI 的本地交互，不覆盖 Codex app、IDE extension 和 cloud task 的完整体验。
- 交互式 agent 可以调用工具，但工具权限受 sandbox、approval、MCP、hooks、配置和当前会话能力限制。
- TUI 中的回答不是事实来源，最终可信度来自源码、命令输出、测试和人工 review。
- `/review` 是专门的 review 工作流；普通 prompt 让 Codex “帮我看看”不等同于 review preset。

## 6. 使用约束

- 工作区 dirty state 必须先看清，不能覆盖用户未确认改动。
- 修改任务要明确范围、验收标准和禁止事项。
- 涉及删除、迁移、大规模格式化、依赖安装、网络访问时必须先解释风险。
- 复杂任务先用 `/plan` 或直接要求“先给计划，不要改文件”。
- 每次文件修改后必须用 `git diff`、`/diff` 或等价方式检查。
- 验证失败时要记录失败命令和失败原因，不能只说“测试没过”。

## 7. 用户入口

- `codex`：打开交互式 TUI。
- `codex "prompt"`：带初始 prompt 启动。
- `/plan`：切到计划模式，先产出执行计划。
- `/diff`：查看 Git diff，包含未跟踪文件。
- `/review`：对 working tree、commit 或 base branch 运行本地代码 review。
- `/status`：查看模型、权限、工作目录、上下文容量等状态。
- `/model`、`/fast`、`/personality`：调整模型和交流方式。
- `/clear`、`/new`、`/compact`、`/resume`：管理会话生命周期和上下文。
- `/permissions`：调整交互式权限模式。
- `!` shell 命令：在 TUI 中排队或运行 shell 命令。

## 8. 典型工作流

读代码：

1. 提供目标和关键路径。
2. 要求 Codex 先列出它会检查的文件。
3. 让 Codex 总结结构和风险点。
4. 要求它引用具体路径和函数名。
5. 把稳定结论整理到笔记。

改代码：

1. 先运行 `git status --short --branch`。
2. 给 Codex 目标、范围、验收标准、禁止事项。
3. 要求先计划。
4. 确认计划后允许编辑。
5. 修改后查看 `/diff` 或 `git diff`。
6. 运行最相关测试。
7. 要求 Codex 总结修改、验证、风险。
8. 自己最终检查并提交。

修失败：

1. 粘贴完整失败命令和关键日志。
2. 要求先定位，不要立即重构。
3. 让 Codex 最小化复现。
4. 修改后重新运行同一命令。
5. 如果失败不同，记录新失败并继续缩小。

## 9. 最佳实践

- Prompt 里同时给“目标、范围、验收、禁止事项、输出格式”。
- 对复杂任务使用两段式：先计划，再执行。
- 对不确定任务使用三段式：先读代码，再提出候选方案，再改。
- 让 Codex 在改文件前说明预期触碰的文件列表。
- 将任务限制到一个功能、一个 bug、一个文档主题或一个测试目标。
- 对生成结论要求“引用文件路径和命令输出”，避免抽象复述。
- 用 `/compact` 前先要求 Codex 总结当前状态、已改文件、验证结果、下一步。
- 如果 Codex 开始发散，直接打断并重述边界。

可复用的交互任务 prompt 模板：

```text
目标：<要完成的单一任务>。
范围：只允许检查或修改 <文件/目录>；不要触碰 <明确排除的范围>。
验收标准：<可检查的 diff、命令结果或行为>。
禁止事项：不要 <删除、安装依赖、访问网络、提交等不允许的操作>。
先阅读相关文件并给出计划，不要立即修改。计划应列出预期文件、验证命令和风险；等待我确认后再执行。
完成时按“修改内容、验证结果、残留风险”三项总结。
```

## 10. 常见错误

- 只说“优化一下”或“完善一下”，没有范围和验收标准。
- Codex 给出计划后不检查，直接让它执行。
- 看到自然语言总结就认为任务完成。
- 修改后不看 diff，不跑测试。
- 一次要求跨多个系统、多个功能和多个文档，导致上下文和验证失控。
- 在已有用户改动的文件上让 Codex 大改，却没有说明哪些改动不可触碰。
- 把 `/clear` 当成清屏，忽略它会开始新 conversation；只想清屏时应使用终端清屏能力。

## 11. 风险和限制

- Codex 可以误读需求或误选文件，尤其在 prompt 模糊时。
- 交互式过程会消耗上下文，长会话可能需要压缩或新线程。
- TUI 中的工具执行仍受 sandbox 和 approval 限制。
- `/review` 的发现要当成高信号建议，不代表没有其他问题。
- 模型回答可能合理但错误；工程结论必须回到源码和验证。

## 12. 验证方式

按任务类型选择验证：

- 文档任务：`git diff --check`、链接和标题检查、人工阅读。
- Rust 代码任务：按 `AGENTS.md` 要求运行 `just fmt`、相关 `just test -p ...`、必要时 `just fix -p ...`。
- 配置任务：运行 `codex -c ...` 或 `/debug-config` 验证实际生效值。
- CLI 行为学习：保存命令、输出摘要和 `/status` 关键字段。
- Review 任务：使用 `/review` 或明确的人工 diff 检查。

完成标准：

- 有清晰 diff。
- 有验证命令及结果。
- 有残留风险。
- 没有无关改动。

## 13. 最小练习

选择一个只改文档的小任务：

1. 打开 Codex TUI。
2. 输入：先阅读目标文件并提出计划，不要改文件。
3. 确认计划后让 Codex 修改。
4. 使用 `/diff` 检查改动。
5. 运行 `git diff --check`。
6. 在 issue #2 记录 prompt、改动文件、验证命令、一个可以改进的 prompt 点。

## 14. 进阶练习

- 用同一个任务比较“直接执行”和“先计划再执行”的差异。
- 故意给一个边界不清的 prompt，观察 Codex 会不会主动澄清或过度行动。
- 让 Codex 修一个失败测试，并要求它每一步都说明假设。
- 使用 `/review` 检查本地 diff，再根据 review 结果继续修。
- 用 `/compact` 压缩长任务前后的上下文，观察后续执行是否仍能保持目标。

## 15. 实验设计

实验目标：找到适合自己的交互式任务控制模板。

对照组：

- A：只给目标。
- B：目标 + 范围 + 验收标准。
- C：目标 + 范围 + 验收标准 + 禁止事项 + 要求先计划。

观察字段：

- Codex 是否先读相关文件。
- 是否触碰无关文件。
- 是否主动运行验证。
- diff 是否容易 review。
- 最终总结是否包含验证和风险。
- 用户需要打断或纠偏几次。

结论写回：

- 把最有效的 prompt 模板写到 `learning-method.md` 或本文件。
- 把交互式问题记录到 issue #2。

## 16. 相关文档

- OpenAI Codex manual：Codex CLI features。
- OpenAI Codex manual：Execution Model and Workflows。
- OpenAI Codex manual：Prompting。
- OpenAI Codex manual：Slash commands in Codex CLI。
- OpenAI Codex manual：Review。
- 本仓库：`docs/getting-started.md`。
- 本仓库：`docs/slash_commands.md`。

## 17. 源码入口

- `codex-rs/cli/src/main.rs`：`codex` 命令进入 TUI 的入口。
- `codex-rs/tui/src/`：终端 UI、composer、slash commands、diff 展示等。
- `codex-rs/tui/src/bottom_pane/chat_composer.rs`：输入区和 prompt 组合相关逻辑。
- `codex-rs/tui/src/bottom_pane/slash_commands.rs`：TUI slash command 入口。
- `codex-rs/core/src/session/`：会话、turn、输入队列、上下文管理。
- `codex-rs/core/src/tasks/regular.rs`：常规任务流。
- `codex-rs/core/src/tools/`：工具注册和调用。

## 18. 待解决问题

- TUI 用户消息如何进入 core session？
- `/plan` 和普通 prompt 的模式差异在哪里实现？
- Codex 如何决定先读哪些文件，再调用哪些工具？
- `/diff` 展示的是 Git 状态还是 Codex 最近 turn 的变更？
- 长会话压缩后，哪些任务事实会保留，哪些容易丢失？

2026-07-10 观察实验：

- 实验通过本地 `codex-tui-observer` MCP 在 `~/codex` 的只读、永不审批子 TUI 中执行；每个命令使用独立新会话，避免前一个命令改变后一个命令的上下文。
- `/plan` 可用，并显示已切换到 Plan mode；当前客户端同时提示 Plan mode 使用 `gpt-5.6-terra xhigh`。这证明它不仅是“要求先给计划”的 prompt 约定，也会改变交互模式和模型设置。
- `/permissions` 打开“Update Model Permissions”选择器，提供 `Ask for approval`、`Approve for me` 和 `Full Access`；实验没有确认任何选项，子会话保持只读。
- `/diff` 在干净工作区中显示 `No changes detected.`，说明它面向工作区 Git 变更，而不依赖本次 turn 已执行修改。
- `/compact` 在新建、没有普通对话 turn 的会话中显示 `No active thread is available.`；因此它需要已有可压缩的活动线程，不能作为任意时刻的无条件“清屏”命令。

## 19. 当前结论

交互式 coding agent 是第一阶段最核心能力。判断一次 Codex 使用是否成功，不看回答是否流畅，而看它是否在受控范围内产生可检查 diff、运行合适验证，并把残留风险讲清楚。2026-07-10 的只读 TUI 观察进一步确认：`/plan` 改变协作模式，`/permissions` 是待确认的权限选择器，`/diff` 检查工作区 Git 状态，而 `/compact` 依赖活动线程。这些命令不能互相替代。

2026-07-10 最小闭环结果：以本文件的 prompt 模板为范围控制工具，只修改了本文件的“最佳实践”部分，并通过 `git diff --check` 验证。对照实验还显示：未提供文件上下文时，计划只能提出通用改进；指定目标文件后，计划明确了文件、验证命令和缺失信息。残留风险是完整 TUI 编辑 turn 会受模型与会话状态影响，因此高风险任务仍应保留人工确认点；本阶段的受控文档改动已形成可复用闭环。
