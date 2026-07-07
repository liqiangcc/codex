# 功能：项目约定

## 1. 一句话说明

用 `AGENTS.md`、项目配置、rules 和相关约定把仓库长期规则交给 Codex，让每次任务都从正确的工程默认值开始。

## 2. 背景和动机

每个仓库都有自己的工作方式：构建命令、测试命令、代码风格、目录职责、禁止事项、review 标准、安全要求。如果每次都靠 prompt 临时说明，容易遗漏，且不同会话之间不一致。项目约定让 Codex 在开始任务前就知道这些规则。

第一阶段要学会区分“长期规则”和“一次性任务”。长期规则进 `AGENTS.md` 或项目配置，一次性目标留在 prompt，重复流程可以升级为 skill。这样学习体系才不会随着聊天记录增长而失控。

## 3. 用户目标

- 能解释 `AGENTS.md` 的作用、发现顺序和优先级。
- 能判断什么内容适合写进 `AGENTS.md`，什么不适合。
- 能理解全局 guidance、项目 guidance、嵌套 guidance 的叠加关系。
- 能用 `/init` 生成初始 `AGENTS.md`，但不会把它当成文档垃圾桶。
- 能把 recurring mistakes 和 recurring review feedback 转化为项目约定。
- 能验证 Codex 是否实际加载并遵守项目约定。

## 4. 适用场景

- 仓库有固定 build/test/lint 命令。
- 子目录有不同语言、框架或测试方式。
- 团队有明确 review guidelines。
- Codex 多次犯同一个错误。
- 需要告诉 Codex 先读哪些文件、不要读哪些生成目录。
- 需要把本学习分支的维护规则固化下来。

不适合：

- 一次性任务目标。
- 大段背景知识或完整产品文档。
- 不可验证的偏好。
- 会频繁变化的临时状态。
- secrets 或私有凭据。

## 5. 功能边界

- `AGENTS.md` 是模型指令和工作约定，不是运行配置；运行配置属于 `config.toml`。
- `AGENTS.md` 不替代测试、lint、类型检查和 CI；它告诉 Codex 应该运行什么，基础设施负责验证。
- Global `AGENTS.md` 适合个人偏好，repo `AGENTS.md` 适合团队和项目规则。
- 嵌套 `AGENTS.md` 或 `AGENTS.override.md` 适合局部目录规则。
- Skills 更适合可复用的多步工作流，MCP 更适合外部系统接入。

## 6. 使用约束

- 规则要短、明确、可执行、可验证。
- 不要把大量产品背景塞进 `AGENTS.md`，会浪费上下文并稀释重点。
- 文件越靠近当前工作目录，约定越具体，后面的 guidance 会覆盖前面的 broad guidance。
- Codex 在每次 run 或 TUI session 开始时构建 instruction chain；修改 guidance 后通常需要新会话或重新启动才能稳定观察。
- 如果 `project_doc_max_bytes` 太小，长 guidance 可能被截断。
- 项目 `.codex/config.toml` 只在 trusted project 中加载。

## 7. 用户入口

- `AGENTS.md`：普通项目指导文件。
- `AGENTS.override.md`：覆盖同层普通 guidance，适合临时或更强规则。
- `~/.codex/AGENTS.md`：个人全局 guidance。
- `~/.codex/AGENTS.override.md`：个人全局临时覆盖。
- `/init`：在当前目录生成 `AGENTS.md` scaffold。
- `project_doc_fallback_filenames`：配置替代 instruction 文件名。
- `project_doc_max_bytes`：控制 instruction chain 字节上限。
- `.codex/config.toml`：项目级运行配置，trusted project 才加载。
- rules / execpolicy：命令允许、提示、阻止策略。

## 8. 典型工作流

新仓库初始化：

1. 先人工阅读 README、docs、现有 `AGENTS.md`。
2. 如果没有 guidance，使用 `/init` 生成草稿。
3. 保留最关键规则：安装、测试、格式化、提交、禁止事项。
4. 让 Codex 用只读任务复述它加载的 instructions。
5. 做一个小改动，观察它是否主动遵守验证规则。

规则维护：

1. 发现 Codex 重复犯错。
2. 判断是否是项目长期规则。
3. 写成一条可执行约定。
4. 放到最靠近适用目录的 `AGENTS.md`。
5. 通过最小任务验证。
6. 在 issue 中记录规则来源和效果。

学习分支约定：

1. `main` 只同步上游。
2. 学习材料只在 `00-study/codex-agent-learning`。
3. 稳定知识进 `study-notes/features/*.md`。
4. 过程记录进 GitHub issue。
5. 每轮学习后执行检查清单。

## 9. 最佳实践

- `AGENTS.md` 写“每次都要遵守”的规则，不写“本次任务要做什么”。
- 用命令形式表达验证规则，例如“修改 Rust 后运行 `just fmt`”。
- 把 review guidelines 单独成段，方便 Codex GitHub review 使用。
- 对 monorepo 使用嵌套 guidance，让子目录规则靠近代码。
- 把重复口头纠正转化为一条规则；不要一次性预设几十条。
- 规则应该能被检查：命令、路径、禁止事项、输出要求。
- 对学习体系，issue 和 feature docs 的职责要写清，避免未来混乱。

## 10. 常见错误

- 把 `AGENTS.md` 当成知识库，越写越长。
- 把一次性需求写进长期项目约定。
- 在全局 guidance 写入只适用于某个仓库的规则。
- 修改 `AGENTS.md` 后继续在旧 session 里验证，误以为 Codex 忽略规则。
- 嵌套规则冲突但没有解释优先级。
- 只写“保持代码质量”，没有具体命令和标准。
- 忘记 `AGENTS.override.md` 的存在，导致 Codex 加载了意外规则。

## 11. 风险和限制

- 过长 guidance 会占用上下文并可能被截断。
- 过期规则会误导 Codex。
- 冲突规则会让 agent 行为不稳定。
- `AGENTS.md` 只是指令，不是安全边界；高风险动作仍靠 sandbox、approval、rules、CI。
- 项目 guidance 可能暴露团队内部流程，公开仓库要避免写入敏感信息。

## 12. 验证方式

验证加载：

```bash
codex --ask-for-approval never "Summarize the current instructions you loaded."
```

验证嵌套：

```bash
codex --cd codex-rs/tui --ask-for-approval never "List the active instruction sources and the most specific rules."
```

验证遵守：

1. 设计一个小任务，要求触发某条规则。
2. 观察 Codex 是否主动提及或执行对应命令。
3. 如果没有，判断是规则不清、session 未刷新，还是任务不触发。

## 13. 最小练习

阅读本仓库根目录 `AGENTS.md`：

1. 把规则分为 Rust、review、TUI、测试、Git、文档限制。
2. 选 3 条最影响日常 Codex 使用的规则。
3. 在 issue #6 记录：规则、适用范围、验证方式。
4. 不修改上游 `AGENTS.md`，除非明确是学习分支需要且不污染上游。

## 14. 进阶练习

- 在临时测试仓库中创建根 `AGENTS.md` 和子目录 `AGENTS.override.md`，验证发现顺序。
- 配置 `project_doc_fallback_filenames`，让 Codex 识别一个替代 guidance 文件。
- 设计一条 recurring mistake 规则，并比较加入前后 Codex 行为。
- 为本学习体系写一段 review guidelines，观察 `/review` 是否能利用它。
- 用 `.codex/config.toml` 和 `AGENTS.md` 同时控制一个任务，区分它们各自作用。

## 15. 实验设计

实验目标：确定哪些学习规则应该长期固化。

候选规则来源：

- 用户反复提醒的要求。
- Codex 反复犯的错误。
- 每轮学习都必须执行的检查命令。
- GitHub issue 和本地文档的同步职责。
- 上游同步流程。

评估维度：

- 是否长期有效。
- 是否适用于整个仓库还是某个目录。
- 是否可验证。
- 是否会泄露信息。
- 是否会增加上下文负担。

产出：

- 一张规则候选表。
- 最终只保留高价值规则，其余放入维护手册或学习方法。

## 16. 相关文档

- OpenAI Codex manual：Custom instructions with AGENTS.md。
- OpenAI Codex manual：Customization。
- OpenAI Codex manual：Config basics。
- OpenAI Codex manual：Rules。
- 本仓库：`AGENTS.md`。
- 本仓库：`docs/agents_md.md`。
- 本仓库：`docs/config.md`。

## 17. 源码入口

- `codex-rs/core/src/agents_md.rs`：AGENTS.md 发现和处理入口。
- `codex-rs/core/src/agents_md_manager.rs`：项目 guidance 管理。
- `codex-rs/core/src/agents_md_tests.rs`：行为测试。
- `codex-rs/exec/tests/suite/agents_md.rs`：exec 模式下 AGENTS.md 测试。
- `codex-rs/config/src/project_root_markers.rs`：项目根目录识别。
- `codex-rs/config/src/config_toml.rs`：相关配置字段。
- `codex-rs/core/src/context/user_instructions.rs`：用户和项目 instructions 注入。

## 18. 待解决问题

- 当前仓库根 `AGENTS.md` 的哪些规则应被学习分支单独引用，哪些只属于上游开发？
- Codex 对 global、project、nested guidance 的合并结果如何在日志中验证？
- `AGENTS.override.md` 是否适合学习分支本地临时规则？
- 长 guidance 达到 `project_doc_max_bytes` 后如何截断，是否有告警？
- Review guidelines 如何影响 `/review` 和 GitHub PR review？

## 19. 当前结论

项目约定的价值在于减少重复说明和降低行为漂移。阶段 1 应先保持 `AGENTS.md` 小而准：写长期、可执行、可验证的规则；一次性目标继续放 prompt；学习过程和结论分别放 issue 与 feature docs。
