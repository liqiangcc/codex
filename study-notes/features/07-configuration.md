# 功能：配置系统

## 1. 一句话说明

用 `config.toml`、profiles、项目配置、命令行覆盖和 feature flags 控制 Codex 的长期默认行为和单次运行差异。

## 2. 背景和动机

Codex 的行为不只由 prompt 决定。模型、reasoning、sandbox、approval、web search、MCP、hooks、skills、plugins、TUI keymap、log_dir、权限 profiles 等都来自配置系统。若不理解配置层级，用户很容易遇到“我明明设置了但没生效”或“这次行为和上次不同”的问题。

第一阶段学习配置系统的目标不是记住所有 key，而是建立判断框架：稳定默认值放配置，一次性任务要求放 prompt，临时实验用 CLI flags 或 `-c`，团队或项目运行设置放 trusted project `.codex/config.toml`，长期仓库指令放 `AGENTS.md`。

## 3. 用户目标

- 能解释 Codex 配置优先级。
- 能判断某个行为应放在 user config、profile、project config、CLI flag、`-c` 还是 prompt。
- 能使用 `/debug-config`、`/status`、`codex doctor` 排查配置是否生效。
- 能用 `codex features` 查看和切换 feature flags。
- 能理解 project config 的信任边界和不可覆盖字段。
- 能为不同学习场景设计最小配置组合。

## 4. 适用场景

- 设置默认模型、reasoning、verbosity。
- 固定 sandbox 和 approval 默认值。
- 配置 web search cached/live/disabled。
- 为深度 review、快速查询、只读学习分别创建 profiles。
- 配置 MCP servers、hooks、skills、plugins。
- 调整 TUI keymap、theme、statusline、title。
- 排查配置冲突和版本升级后的 unknown keys。

不适合把具体任务目标写进配置。配置解决“Codex 默认怎么运行”，prompt 解决“这次要做什么”。

## 5. 功能边界

- `~/.codex/config.toml` 是个人默认配置。
- `~/.codex/<profile>.config.toml` 是 profile layer，通过 `--profile` 选择。
- `.codex/config.toml` 是项目配置，只有 trusted project 加载。
- CLI flags 和 `-c key=value` 是单次运行最高优先级覆盖。
- `AGENTS.md` 不是配置文件，它是项目 guidance。
- `requirements.toml` 和 managed configuration 可能限制用户配置，普通用户不能用本地配置绕过。

## 6. 使用约束

- 配置优先级从高到低：CLI flags/`--config`、project config、profile、user config、system config、built-in defaults。
- 项目 `.codex/config.toml` 不能覆盖 provider auth、credentials、profile、notification、telemetry 等敏感或机器本地设置。
- project `.codex/` layers、project-local hooks、project-local rules 只在 trusted project 加载。
- `--config` 的值按 TOML 解析；shell quoting 错误会导致意外字符串。
- profile 文件使用 `~/.codex/profile-name.config.toml`，不是在 user config 里写旧式 `[profiles.name]`。
- 开启 experimental feature 要记录原因，避免长期默认被污染。

## 7. 用户入口

- `~/.codex/config.toml`。
- `.codex/config.toml`。
- `~/.codex/<profile>.config.toml`。
- `codex --profile <name>`。
- `codex -c key=value` / `codex --config key=value`。
- 专用 flags：`--model`、`--sandbox`、`--ask-for-approval`、`--search`、`--enable`、`--disable`、`--strict-config`。
- `/debug-config`：查看配置层和 requirements diagnostics。
- `/status`：查看当前会话实际生效状态。
- `codex features list|enable|disable`。
- `/experimental`：TUI 中切换实验功能。
- `codex debug models`：查看模型 catalog。

## 8. 典型工作流

临时实验：

1. 不改配置文件。
2. 用 CLI flags 或 `-c` 运行一次。
3. 用 `/status` 或 `/debug-config` 验证。
4. 确认稳定后再写入 user config 或 profile。

创建 profile：

1. 写 `~/.codex/deep-review.config.toml`。
2. 只写和默认配置不同的 key。
3. 用 `codex --profile deep-review` 测试。
4. 用 `/debug-config` 确认 layer 顺序。
5. 在学习笔记记录 profile 用途。

项目配置：

1. 只在 trusted project 中使用。
2. 放项目共享且不涉及凭据重定向的设置。
3. 不把个人 key、token、telemetry command 写入项目配置。
4. 和 `AGENTS.md` 分工：config 管运行参数，AGENTS 管工程规则。

## 9. 最佳实践

- 稳定默认放 user config，任务级差异用 CLI flags。
- 把高风险设置保留为显式命令，不写入默认配置。
- 为学习创建少量 profiles，例如 `read-only-study`、`deep-review`、`exec-automation`。
- 用 `--strict-config` 检查上游升级后是否出现未知配置项。
- 用 `/debug-config` 排查“为什么没生效”，不要靠猜。
- 配置变更后记录验证命令和观察结果。
- 对 feature flags 使用“启用理由 + 验证结果 + 是否保留”的记录方式。

## 10. 常见错误

- 把 prompt 要求写进 config，导致所有任务都受影响。
- 忘记 `-c` 优先级最高，临时覆盖后误判 user config。
- 误把 profile 写成旧式 `[profiles.name]`。
- 在项目 config 里放 provider、credential 或 notification 相关本地设置。
- 启用 experimental feature 后忘记关闭。
- 不使用 `/debug-config`，只看配置文件推断实际状态。
- 混用 permission profiles 和旧 sandbox 设置，没有明确哪套生效。

## 11. 风险和限制

- 配置过度复杂会显著增加排查成本。
- 项目配置可能被不可信仓库利用，因此 Codex 有 trusted project 边界。
- 错误的 shell quoting 会让 `-c` 写入完全不同的类型。
- Managed requirements 可能让本地配置看似正确但被限制。
- feature flags 和 experimental 能力可能变化，学习结论需要定期复核。

## 12. 验证方式

基础验证：

```text
/status
/debug-config
```

命令验证：

```bash
codex --strict-config "Summarize active config constraints"
codex features list
codex debug models --bundled
```

配置覆盖实验：

```bash
codex -c model='"gpt-5.5"' -c web_search='"disabled"' "Report the active model and web search mode"
```

通过标准：

- 能说明每个生效值来自哪一层。
- 能解释为什么某个 project config 被加载或被忽略。
- 能把临时覆盖和持久默认分清。

## 13. 最小练习

完成一次配置层级观察：

1. 打开 TUI。
2. 运行 `/status`。
3. 运行 `/debug-config`。
4. 运行 `codex features list`。
5. 在 issue #7 记录：user config 路径、是否有 project config、当前 sandbox、web search、feature flags 中最需要关注的一项。

## 14. 进阶练习

- 创建 `~/.codex/read-only-study.config.toml`，设置 read-only 和 on-request，使用 `--profile` 启动。
- 用 `-c web_search='"disabled"'` 和 `--search` 对比 web search 生效情况。
- 使用 `--strict-config` 人为测试一个 unknown key，在临时 config 中观察错误。
- 在临时 trusted project 配置一个 `.codex/config.toml`，验证 closest wins。
- 修改 `[features]` 后用 `/experimental` 和 `codex features list` 对比。

## 15. 实验设计

实验目标：建立“配置放置决策表”。

候选设置：

- 默认模型。
- review_model。
- sandbox。
- approval。
- web_search。
- MCP server。
- hook。
- TUI keymap。
- statusline。

决策维度：

- 是否个人偏好。
- 是否项目共享。
- 是否临时实验。
- 是否涉及凭据或机器本地路径。
- 是否有安全风险。
- 是否需要 mobile issue 记录。

产出：

- 一张表：设置项、推荐位置、验证命令、风险。

## 16. 相关文档

- OpenAI Codex manual：Config basics。
- OpenAI Codex manual：Advanced Configuration。
- OpenAI Codex manual：Configuration Reference。
- OpenAI Codex manual：Model selection。
- OpenAI Codex manual：Permissions。
- 本仓库：`docs/config.md`。
- 本仓库：`docs/example-config.md`。

## 17. 源码入口

- `codex-rs/config/src/loader/`：配置层加载。
- `codex-rs/config/src/config_toml.rs`：TOML 配置结构。
- `codex-rs/config/src/merge.rs`：配置合并。
- `codex-rs/config/src/overrides.rs`：命令行覆盖。
- `codex-rs/config/src/profile_toml.rs`：profile 相关解析。
- `codex-rs/config/src/strict_config.rs`：strict config 检查。
- `codex-rs/features/src/`：feature flags。
- `codex-rs/tui/src/debug_config.rs`：TUI debug config 展示。

## 18. 待解决问题

- 当前版本中哪些配置仍在 core crate，哪些已经迁移到 `codex-rs/config`？
- `/debug-config` 展示的 layer 和实际 merge 逻辑是否完全对应？
- feature flags 的成熟度如何映射到学习优先级？
- permission profiles 与 `sandbox_mode` 同时配置时如何提示用户？
- 学习分支是否需要一个推荐 profile 示例，还是只记录原则？

## 19. 当前结论

配置系统的学习重点是优先级和职责边界。阶段 1 不追求记住所有 key，而要能判断“这个行为应该放在哪里、如何临时覆盖、如何验证生效、如何避免长期配置污染”。
