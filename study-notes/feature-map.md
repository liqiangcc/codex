# Codex CLI 功能地图

这个文件是阶段 1 的功能索引和覆盖矩阵。稳定、详细的功能解释放在 `study-notes/features/*.md`；GitHub Issues 用作过程看板和手机端记录；本文件只维护功能状态、阅读入口和阶段覆盖范围。

## 阅读顺序

1. 先看“功能索引”，选择一个 `Todo` 或 `In Progress` 功能。
2. 打开对应“详细文档”，理解背景、边界、入口、练习和实验设计。
3. 打开对应 GitHub issue，查看当前任务、过程记录和下一步。
4. 做最小练习或真实任务，把稳定结论写回详细文档。
5. 更新 issue 顶部摘要，保持手机上一屏可读。
6. 完成后同步本文件状态，并关闭或标记对应 issue。

## 状态说明

- `Todo`：还没开始。
- `In Progress`：正在学习。
- `Need experiment`：需要最小实验验证。
- `Need source reading`：需要进入源码确认实现。
- `Done`：已形成可复用结论，并完成阶段 1 成功标准。

## 功能索引

| 功能 | 状态 | Issue | 详细文档 |
| --- | --- | --- | --- |
| 安装、登录、启动 | Done | [#1](https://github.com/liqiangcc/codex/issues/1) | [01-install-auth-start.md](features/01-install-auth-start.md) |
| 交互式 coding agent | Done | [#2](https://github.com/liqiangcc/codex/issues/2) | [02-interactive-coding-agent.md](features/02-interactive-coding-agent.md) |
| 非交互执行 | Done | [#3](https://github.com/liqiangcc/codex/issues/3) | [03-non-interactive-exec.md](features/03-non-interactive-exec.md) |
| 任务上下文 | Done | [#4](https://github.com/liqiangcc/codex/issues/4) | [04-task-context.md](features/04-task-context.md) |
| 权限和安全 | Done | [#5](https://github.com/liqiangcc/codex/issues/5) | [05-permissions-and-safety.md](features/05-permissions-and-safety.md) |
| 项目约定 | Done | [#6](https://github.com/liqiangcc/codex/issues/6) | [06-project-guidance.md](features/06-project-guidance.md) |
| 配置系统 | Done | [#7](https://github.com/liqiangcc/codex/issues/7) | [07-configuration.md](features/07-configuration.md) |
| 工具和扩展 | Done | [#8](https://github.com/liqiangcc/codex/issues/8) | [08-tools-and-extensions.md](features/08-tools-and-extensions.md) |
| 调试和诊断 | Done | [#9](https://github.com/liqiangcc/codex/issues/9) | [09-debugging-and-diagnostics.md](features/09-debugging-and-diagnostics.md) |
| Git 和协作 | Done | [#10](https://github.com/liqiangcc/codex/issues/10) | [10-git-and-collaboration.md](features/10-git-and-collaboration.md) |

## 阶段 1 功能覆盖清单

阶段 1 不为每个细功能单独开一级卡片，而是把它们映射到上面的 10 张功能卡片。这样学习路线保持稳定，同时能确认 Codex CLI 的主要使用面没有漏掉。

| Codex 细功能 | 所属功能卡片 | 阶段 1 深度 | 备注 |
| --- | --- | --- | --- |
| 安装 CLI、版本确认、启动 TUI | 安装、登录、启动 | 深入 | 建立所有学习任务的运行基线。 |
| `codex login`、`logout`、认证方式 | 安装、登录、启动 | 深入 | 重点理解登录状态、凭据、headless 登录和故障排查。 |
| `codex doctor`、环境诊断 | 安装、登录、启动；调试和诊断 | 深入 | 既是启动检查，也是异常排查入口。 |
| shell completions | 安装、登录、启动 | 浅用 | 提升日常命令输入效率，不作为行为核心。 |
| 交互式 TUI、composer、初始 prompt | 交互式 coding agent | 深入 | 第一阶段核心能力。 |
| 让 Codex 读代码、改代码、跑命令、跑测试 | 交互式 coding agent | 深入 | 每次学习都要落到可验证任务。 |
| `/plan`、先计划再执行 | 交互式 coding agent | 深入 | 用于控制复杂任务风险。 |
| Prompt 写法、验收标准、范围控制 | 任务上下文；交互式 coding agent | 深入 | 直接决定 Codex 输出质量。 |
| TUI 主题、keymap、vim、statusline、title | 交互式 coding agent；配置系统 | 浅用 | 属于效率功能，阶段 1 不深入实现。 |
| `codex exec`、一次性 prompt | 非交互执行 | 深入 | 学会和交互式模式做取舍。 |
| `codex exec --json`、JSONL 输出 | 非交互执行 | 基础 | 先理解自动化价值和事件流消费方式。 |
| `codex exec --output-schema`、`-o` | 非交互执行 | 基础 | 用于结构化输出和脚本集成。 |
| `codex exec resume` | 非交互执行；任务上下文 | 基础 | 关注历史上下文如何延续。 |
| `resume`、`/resume`、继续会话 | 任务上下文 | 基础 | 学会恢复上下文，避免重复说明。 |
| `/new`、`/clear`、`/compact` | 任务上下文 | 基础 | 重点理解什么时候重置、清屏或压缩上下文。 |
| `/fork`、`/side`、分支对话 | 任务上下文 | 浅用 | 用于探索替代方案，暂不深入实现。 |
| `/goal`、目标跟踪 | 任务上下文 | 基础 | 用于长任务保持目标，但阶段 1 只学习使用边界。 |
| 文件路径、错误日志、图片输入 | 任务上下文 | 基础 | 图片输入作为可选上下文能力。 |
| Web search、`--search`、`web_search` 配置 | 任务上下文；权限和安全；配置系统 | 基础 | 重点理解新鲜度、prompt injection 风险和网络边界。 |
| sandbox、approval、`/permissions` | 权限和安全 | 深入 | 第一阶段必须掌握。 |
| permission profiles | 权限和安全；配置系统 | 基础 | 理解更细粒度文件系统和网络权限。 |
| `--yolo`、Full Access | 权限和安全 | 基础 | 只理解风险，不作为默认使用方式。 |
| `execpolicy`、rules | 权限和安全 | 基础 | 先知道它解决命令策略问题，后续再源码深入。 |
| network 权限和外部访问 | 权限和安全；配置系统 | 基础 | 和 web search、MCP、命令执行一起理解。 |
| `AGENTS.md`、`/init` | 项目约定 | 深入 | 适合沉淀仓库长期规则。 |
| `AGENTS.override.md`、嵌套 guidance | 项目约定 | 基础 | 理解发现顺序、覆盖关系和上下文上限。 |
| custom prompts、rules | 项目约定；工具和扩展 | 基础 | custom prompts 已不作为优先方向，复用流程优先考虑 skills。 |
| `config.toml`、profiles、`-c key=value` | 配置系统 | 深入 | 重点理解配置优先级和适用范围。 |
| model、reasoning、`/model`、`review_model` | 配置系统 | 基础 | 学会选择和检查，不做模型评测。 |
| feature flags、`codex features`、`/experimental` | 配置系统 | 基础 | 只启用确实需要验证的功能。 |
| `log_dir`、OTel、诊断配置 | 配置系统；调试和诊断 | 浅用 | 第一阶段只掌握定位入口。 |
| MCP、`codex mcp`、`/mcp` | 工具和扩展 | 基础 | 重点理解外部工具接入边界。 |
| skills、`/skills`、`$skill` | 工具和扩展 | 基础 | 用于复用任务工作流。 |
| hooks、`/hooks` | 工具和扩展；权限和安全 | 基础 | 重点关注 trust、事件和生命周期。 |
| plugins、`codex plugin`、`/plugins` | 工具和扩展 | 基础 | 先理解分发和安装，不做插件开发。 |
| apps/connectors、`/apps` | 工具和扩展 | 浅用 | 当前阶段只了解概念、授权和数据边界。 |
| subagents、`/agent` | 工具和扩展；任务上下文 | 可选 | 阶段 1 不作为必学项，后续作为高级 agent 主题。 |
| `/status`、`/debug-config` | 调试和诊断 | 深入 | 用于确认模型、权限、上下文和配置；学习流程禁止执行 `/usage`，避免触发额度重置。 |
| `codex debug models` | 调试和诊断；配置系统 | 基础 | 用于理解模型目录和排查模型问题。 |
| `/ps`、`/stop`、后台命令状态 | 调试和诊断 | 基础 | 用于处理长时间运行或卡住的命令。 |
| `/feedback` | 调试和诊断 | 浅用 | 只在需要反馈问题时使用，提交前要脱敏。 |
| Git dirty state、diff、commit、push | Git 和协作 | 深入 | 学习分支的日常闭环。 |
| `/diff`、查看本地变更 | Git 和协作 | 深入 | 每次写文件后都要检查。 |
| `/review`、本地代码 review | Git 和协作 | 基础 | 先掌握使用场景，后续再读实现。 |
| GitHub Issues 学习看板 | Git 和协作 | 深入 | 手机端过程记录和状态跟踪。 |
| PR、review comments、上游贡献 | Git 和协作 | 基础 | 阶段 1 只掌握概念，不向上游发 PR。 |
| Codex IDE extension | 暂不纳入阶段 1 | 暂不深入 | 后续可作为单独阶段。 |
| Codex app、Codex cloud | 暂不纳入阶段 1 | 暂不深入 | 先把 CLI 学扎实。 |
| app-server、SDK、GitHub Action、remote-control | 暂不纳入阶段 1 | 暂不深入 | 属于程序化接口或开发者扩展阶段。 |

如果某个细功能无法自然归入这张表中的任何卡片，再考虑新增一级功能卡片。

## 详细文档模板

功能详细文档按“先理解、再使用、再验证、再深入”的顺序组织：

```markdown
# 功能：名称

## 1. 一句话说明
## 2. 背景和动机
## 3. 用户目标
## 4. 适用场景
## 5. 功能边界
## 6. 使用约束
## 7. 用户入口
## 8. 典型工作流
## 9. 最佳实践
## 10. 常见错误
## 11. 风险和限制
## 12. 验证方式
## 13. 最小练习
## 14. 进阶练习
## 15. 实验设计
## 16. 相关文档
## 17. 源码入口
## 18. 待解决问题
## 19. 当前结论
```

这个顺序的原则是：先让读者理解功能解决什么问题，再给出使用边界和操作入口，然后进入验证和练习，最后才转向文档、源码和未解问题。

## Issue 摘要原则

GitHub issue 不承载完整功能文档，只作为手机端任务入口：

- 顶部摘要必须一屏可读。
- 必须链接对应详细文档。
- 当前任务和下一步必须明确。
- 过程记录可以粗糙，但稳定结论要整理回详细文档。
- issue 完成状态要和本文件功能索引保持一致。

## 阶段完成标准

阶段 1 完成时，应满足：

- 10 个一级功能都有详细文档。
- 每个详细文档都有背景、场景、边界、约束、入口、工作流、最佳实践、风险、验证、练习、实验、文档入口、源码入口、待解决问题和当前结论。
- 10 个 feature issue 都能从手机端快速看到状态、当前任务、下一步和详细文档链接。
- `checks.md` 能检查详细文档数量、关键章节、issue 链接和占位符。
- `maintenance.md` 明确本地文档、issue、实验和上游同步的维护职责。
