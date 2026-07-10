# Plugin 分发

## 分发链路

```text
personal marketplace entry
  -> source plugin manifest
  -> cachebuster version reinstall
  -> immutable cache directory
  -> active plugin contributes Skill roots and MCP servers
  -> disable/uninstall removes effective contributions
```

## 源码证据

- `codex-rs/core-plugins/src/manifest.rs::load_plugin_manifest` 解析 `.codex-plugin/plugin.json`。
- `codex-rs/core-plugins/src/loader.rs::load_plugins_from_layer_stack` 加载配置中的插件；`load_plugin_skills` 和 `load_plugin_mcp_servers` 分别解析组件。
- `codex-rs/core-plugins/src/store.rs::PluginStore::install_with_version` 建立版本化安装，`uninstall` 提供删除边界。
- `codex-rs/plugin/src/load_outcome.rs::LoadedPlugin::is_active` 和 `PluginLoadOutcome::effective_*` 确保 disabled plugin 不贡献能力。
- `codex-rs/core-plugins/src/loader.rs::refresh_non_curated_plugin_cache_force_reinstall` 是本地非 curated 插件缓存刷新的源码入口。

## Manifest 与版本

`codex-tui-observer` manifest 声明：

- `skills: ./skills/`
- `mcpServers: ./.mcp.json`
- UI metadata 和一个不会扩大权限的默认 prompt
- 基础版本 `0.1.0`，开发升级后为 `0.1.0+codex.20260710145859`

升级遵循 plugin-creator 流程：更新单一 cachebuster，读取 personal marketplace 名称，运行 `codex plugin add codex-tui-observer@personal`，再用 `codex plugin list` 检查状态。

## 行为证据

- Skill validator：通过。
- Plugin validator：通过。
- Python server compile：通过。
- 重装输出给出缓存目录 `/root/.codex/plugins/cache/personal/codex-tui-observer/0.1.0+codex.20260710145859`。
- 缓存目录同时包含 manifest、`.mcp.json`、server、README、Skill 和 `agents/openai.yaml`。
- `codex plugin list` 显示 `installed, enabled` 与同一版本。
- 全新子线程 `/skills` 显示新 Skill，证明不是只更新 source 而未刷新 cache。

最终卸载/恢复实验：

- `codex plugin remove codex-tui-observer@personal --json` 成功后，`codex plugin list` 显示 `not installed`。
- 新启动的 read-only 子 Codex 进入 `/skills` 并搜索显示名，`Observe Codex Study` 不可见；子进程仍被显式 stop。
- `codex plugin add codex-tui-observer@personal` 恢复同一 cachebuster 版本，状态重新变为 `installed, enabled`。
- 再启动一个全新子 Codex；完整列表首屏未显示该条目，但在 `/skills` 搜索 `Observe Codex Study` 后命中，证明是列表位置变化而非加载失败；子进程随后 stop。

## 禁用、卸载和凭据边界

- 禁用插件后，Skill root 与 MCP server 都不应进入 effective contributions。
- 卸载命令只针对 `codex-tui-observer@personal`；卸载与恢复均已实际验证，最终保留启用状态用于后续学习，但其行为仍是 read-only。
- 插件不包含 token、远程 URL、生产凭据或付费 API。
- `.mcp.json` 当前使用本机绝对 server 路径，因此是 personal plugin，不作为可移植公共市场包发布。
