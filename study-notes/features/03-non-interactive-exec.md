# 功能：非交互执行

## 1. 一句话说明

用 `codex exec` 在没有 TUI 多轮交互的情况下运行 Codex，让它适配脚本、CI、批处理和一次性分析任务。

## 2. 背景和动机

交互式 TUI 适合探索和协作，但很多任务更适合一次性运行：总结日志、生成报告、检查变更、对 CI 输出做 triage、按固定格式产出 JSON、在自动化流程中生成 patch。`codex exec` 把 Codex 变成命令行工具链中的一环。

非交互模式的关键不是“少一个 UI”，而是更强的前置约束：prompt 必须完整，权限必须预设，输出必须可消费，失败必须可检测。它适合低歧义、可复现、可验证的任务；不适合依赖大量澄清的问题。

## 3. 用户目标

- 能判断任务应该用 TUI 还是 `codex exec`。
- 能写出适合非交互运行的完整 prompt。
- 能控制 sandbox、approval、工作目录、stdin、输出文件和 JSONL 输出。
- 能把命令输出、日志或 diff pipe 给 Codex。
- 能从退出码、stdout、stderr、JSONL 事件和 Git diff 判断执行是否成功。

## 4. 适用场景

- 在 CI 或本地脚本中总结失败日志。
- 对仓库结构、风险点、测试结果做一次性报告。
- 自动生成 changelog、release notes、PR comment 草稿。
- 对小范围改动运行固定修复或检查。
- 用 JSON Schema 获取结构化结果。
- 分阶段自动化：一次 `exec` 发现问题，下一次 `exec resume` 继续修复。

不适合：

- 需求尚不清楚，需要来回澄清。
- 修改范围大、风险高、需要频繁人工判断。
- 用户无法检查输出或 diff。
- 需要长期会话状态和复杂分支探索。

## 5. 功能边界

- `codex exec` 是非交互任务执行入口，不是 TUI 的完整替代品。
- 默认输出模型最终消息到 stdout，进度走 stderr；启用 `--json` 后 stdout 变为 JSONL 事件流。
- 默认 sandbox 是 read-only；如果要编辑，需要显式设置 `--sandbox workspace-write`。
- 自动化认证、API key、CI runner 安全是独立关注点，不能只靠 `codex exec` 本身解决。
- `codex exec resume` 继续已有非交互或会话上下文，但仍需要清晰的新指令。

## 6. 使用约束

- 新脚本优先使用显式 `--sandbox`，不要依赖隐含默认。
- 自动化中使用最小权限，能 read-only 就不要 workspace-write。
- 只有在隔离 runner 或容器里才考虑 `danger-full-access`。
- 不要在公共或不可信仓库的构建脚本中暴露 `OPENAI_API_KEY` 或 `CODEX_API_KEY`。
- 如果启用 required MCP server，必须处理它初始化失败导致的非零退出。
- 如果跳过 Git 仓库检查，必须确认运行目录和写入范围足够安全。

## 7. 用户入口

- `codex exec "prompt"`：运行一次非交互任务。
- `codex e "prompt"`：`exec` 的 alias。
- `codex exec --json "prompt"`：输出 JSONL 事件。
- `codex exec -o result.md "prompt"`：把最终消息写入文件。
- `codex exec --output-schema schema.json -o result.json "prompt"`：按 JSON Schema 输出最终结构化结果。
- `codex exec resume --last "prompt"`：继续最近一次会话。
- `codex exec resume <SESSION_ID> "prompt"`：继续指定会话。
- `--ephemeral`：不持久化 session rollout 文件。
- `--ignore-user-config`：不加载用户配置。
- `--ignore-rules`：跳过用户和项目 execpolicy rules。
- `--sandbox read-only|workspace-write|danger-full-access`：控制执行权限。

## 8. 典型工作流

只读分析：

1. 进入仓库。
2. 写清分析目标和输出格式。
3. 运行 `codex exec "..."`。
4. 检查 stdout 最终回答。
5. 如果要保存，使用 `tee` 或 `-o`。

日志 triage：

1. 用命令生成日志。
2. pipe 给 Codex，prompt 作为指令。
3. 要求输出 root cause、证据、下一步。
4. 保存结果给 issue 或 PR comment。

```bash
npm test 2>&1 | codex exec "Summarize the failing tests and propose the smallest likely fix"
```

小范围自动修复：

1. 确认工作区干净。
2. 用 `--sandbox workspace-write`。
3. prompt 限制文件范围和验证命令。
4. 执行后检查退出码、diff、测试。
5. 必要时用 `codex exec resume --last` 继续。

## 9. 最佳实践

- Prompt 要自包含：目标、输入来源、范围、输出格式、失败处理。
- 让 Codex 输出“做了什么、怎么验证、还剩什么风险”。
- 把复杂任务拆成多个 `exec` 步骤，每一步有明确产物。
- 自动化读取 stdout 时，记住普通模式 stdout 主要是最终消息，stderr 才是进度。
- 机器消费输出时优先 `--json` 或 `--output-schema`。
- 写文件前先在 read-only 模式跑一次分析，确认方向后再 workspace-write。
- CI 中优先使用专门的 Codex GitHub Action，而不是手写暴露 API key 的 shell 步骤。

## 10. 常见错误

- 把 TUI prompt 原样搬到 `exec`，但没有补足上下文。
- 以为非交互模式默认可以改文件；实际默认 read-only。
- 在脚本里不检查退出码。
- 用普通自然语言输出驱动自动化，却没有 schema 或 JSONL。
- 把大量无关日志 pipe 给 Codex，导致关键信号被稀释。
- 在自动化环境中让 Codex 拥有过宽网络和文件权限。
- 用 `--ask-for-approval never` 后误以为操作一定能成功；sandbox 仍可能限制行动。

## 11. 风险和限制

- 缺少交互澄清，prompt 质量直接决定结果。
- 自动化会放大权限配置错误。
- 结构化输出仍需要校验 schema 和业务语义。
- JSONL 事件名和字段可能随版本演进，脚本要保守解析。
- 如果任务产生 diff，仍必须由 Git 和测试闭环确认。

## 12. 验证方式

非交互任务至少检查：

```bash
git status --short --branch
codex exec --sandbox read-only "Summarize this repository in 5 bullets"
echo $?
git diff --stat
```

写入任务额外检查：

```bash
git diff --check
git diff --stat
```

JSONL 任务检查：

```bash
codex exec --json "Summarize this repository" | jq -r '.type'
```

通过标准：

- 退出码符合预期。
- stdout/stderr 分工清楚。
- 如果有 diff，范围符合 prompt。
- 输出可以被人或脚本稳定消费。

## 13. 最小练习

完成两个对照任务：

1. `codex exec --sandbox read-only "Summarize study-notes in 5 bullets"`。
2. `codex exec --json --sandbox read-only "Summarize study-notes in 5 bullets" | jq -r '.type'`。

在 issue #3 记录：

- 普通输出和 JSONL 输出差异。
- 是否产生 diff。
- 退出码。
- 哪种输出更适合手机查看，哪种更适合脚本。

## 14. 进阶练习

- 用 `git diff --stat | codex exec "Explain the changed areas and risk"` 做一次 diff triage。
- 用 `--output-schema` 让 Codex 输出 `{summary, risks, next_steps}`。
- 运行一次 `codex exec --ephemeral`，观察 session 是否出现在可恢复列表中。
- 用 `codex exec resume --last` 继续上一次分析，验证历史上下文是否被保留。
- 用 `--ignore-user-config` 对比用户配置是否影响输出和权限。

## 15. 实验设计

实验目标：确定哪些学习任务可以脚本化，哪些必须交互式。

任务样本：

- 仓库结构总结。
- 测试失败日志总结。
- 小文档改动。
- 小代码修复。

对照维度：

- TUI vs `codex exec`。
- read-only vs workspace-write。
- 普通 stdout vs JSONL。
- prompt 含验收标准 vs 不含验收标准。

观察字段：

- 是否需要澄清。
- 结果是否可直接使用。
- diff 是否超范围。
- 验证是否自动执行。
- 用户需要后续修正几次。

## 16. 相关文档

- OpenAI Codex manual：Non-interactive mode。
- OpenAI Codex manual：CLI command reference，`codex exec`。
- OpenAI Codex manual：Agent approvals & security。
- OpenAI Codex manual：Authentication and sessions。
- 本仓库：`docs/exec.md`。
- 本仓库：`docs/sandbox.md`。

## 17. 源码入口

- `codex-rs/cli/src/main.rs`：`exec` 子命令入口。
- `codex-rs/exec/src/`：非交互执行 crate。
- `codex-rs/exec/tests/suite/`：exec 行为集成测试。
- `codex-rs/core/src/session/`：非交互仍会进入核心 session/turn 流。
- `codex-rs/core/src/stream_events_utils.rs`：事件流相关处理。
- `codex-rs/rollout/src/` 和 `codex-rs/rollout-trace/src/`：会话记录和 trace 相关入口。

## 18. 待解决问题

- `codex exec` 和 TUI 在初始上下文注入上有哪些差异？
- JSONL 事件中哪些字段适合长期脚本依赖，哪些只适合调试观察？
- `--ephemeral` 对 resume、日志和诊断有什么影响？
- 默认 read-only 为什么适合自动化，它在哪些任务上会显得不够用？

## 19. 当前结论

非交互执行适合“目标清楚、输入明确、输出可验证”的任务。它的核心能力不是省掉 TUI，而是把 Codex 纳入命令行自动化；因此必须更重视 prompt 完整性、权限最小化、退出码、结构化输出和 diff 检查。
