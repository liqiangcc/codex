## 目标

定位根因，选择最小、可维护且不复制完整 Hook schema 的设计。

## 完成证据

- 追踪 `RawPluginManifestHooks`、`resolve_manifest_hooks`、`PluginManifestHooks` 和 Python allowlist。
- 比较移除示例、只放宽 allowlist、完整 preflight 与复制全 schema 四个方案。
- 完成 app-server/raw events/CLI/config/rollout breaking surface 检查。

详细记录：[根因与变更设计](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/contributions/root-cause-and-design.md)

## 结论

选择接受四种 core shape，并只做路径、文件和顶层对象 preflight。
