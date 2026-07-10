# Skills 生命周期

## 生命周期链路

```text
plugin manifest skills path
  -> plugin loader resolves skill roots
  -> SkillsService merges roots and config rules
  -> loader reads bounded SKILL.md metadata
  -> explicit $mention or implicit match selects skill
  -> full instructions are injected for the turn
```

`agents/openai.yaml` 是 UI/产品元数据，`SKILL.md` frontmatter 的 `name` 和 `description` 才是 agent 发现与触发的核心。完整正文不应常驻所有 turn；它在选择后才进入上下文。

## 源码证据

- `codex-rs/core-skills/src/root_loader.rs::load_and_merge_skill_roots` 合并普通 root 和 plugin skill snapshot。
- `codex-rs/core-skills/src/service.rs::SkillsService::snapshot_for_config` 加载并缓存配置对应的 Skill 集合，`clear_cache` 提供显式失效边界。
- `codex-rs/core-skills/src/invocation_utils.rs::detect_implicit_skill_invocation_for_command` 处理命令中的隐式调用匹配。
- `codex-rs/core-plugins/src/loader.rs::load_plugin_skills` 把插件 manifest/default root 转换为可用 Skill。
- `codex-rs/core-skills/src/service_tests.rs::skills_for_config_disables_plugin_skills_by_name` 证明插件 Skill 可按名称禁用。

## 实现

本地插件新增 `skills/observe-codex-study/`：

- `SKILL.md` 只包含工作流指令，不附带脚本、资产或重复参考资料。
- 触发场景是学习 slash command、验证交互行为和收集可重复 TUI 证据。
- 安全规则明确拒绝 `/usage`、登出、账户切换、额度重置、认证变更、任意 shell 和破坏性 Git。
- `agents/openai.yaml` 提供 `Observe Codex Study` 显示名、短描述和带 `$observe-codex-study` 的默认 prompt。

## 行为实验

验证命令：

- `quick_validate.py .../observe-codex-study`：通过。
- `validate_plugin.py /root/plugins/codex-tui-observer`：通过。
- 重装插件后启动全新子 Codex，输入 `/skills`，选择 `List skills`：列表出现 `Observe Codex Study — Record safe, read-only Codex TUI learning evidence`。

第一次只输入 `/skills` 后直接查显示名得到 `false`，原因是界面还停留在“List skills / Enable/Disable Skills”二级菜单；再按一次 Enter 才进入列表。这个失败验证了观察脚本必须等待正确 UI 状态，不能只按固定 sleep 判定。

## 上下文与清理边界

- Skill 正文约束为单一流程，避免把终端 transcript 注入上下文。
- session ID 仅在工具调用间传递，不写入笔记。
- 插件禁用时其 Skill root 不进入 effective roots；卸载插件即可移除 Skill。
- 更新 Skill 后必须使用新 cachebuster 重装，并在新线程验证；当前线程不会热加载新 Skill。
