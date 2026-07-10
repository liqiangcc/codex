# 根因与变更设计

## 根因链路

```text
core manifest 增加/支持 hooks 多形态
  -> plugin-creator 示例同步展示 hooks
  -> Python preflight validator 的 allowlist/companion validation 未同步
  -> 文档末尾保留旧的“拒绝 hooks”说明
```

关键源码：

- `codex-rs/core-plugins/src/manifest.rs::RawPluginManifestHooks`
- `codex-rs/core-plugins/src/manifest.rs::resolve_manifest_hooks`
- `codex-rs/plugin/src/manifest.rs::PluginManifestHooks`
- `codex-rs/skills/src/assets/samples/plugin-creator/scripts/validate_plugin.py::validate_manifest_shape`

## 候选方案

1. 从示例移除 `hooks`：改动小，但与 core 已支持能力相反，继续让 creator 落后。
2. 只把 `hooks` 加入 allowlist：能绕过原错误，但会让缺失文件、路径逃逸和损坏 JSON 静默通过。
3. 让 validator 接受 core 的四种 shape，并做与现有 MCP/App preflight 同级的路径和顶层对象校验：选择此方案。
4. 在 Python 中完全重写 Rust Hook schema：过重、容易再次漂移，不适合本次最小修复。

## 最小设计

- `allowed_keys` 增加 `hooks`。
- 新增 `validate_manifest_hooks` 分发 string、string array、object、object array。
- 文件路径必须以 `./` 开始、留在 plugin archive 内、实际存在并包含有效 JSON。
- Hook 顶层只允许 `description`/`hooks`，`description` 非空，`hooks` 为 object。
- 不复制 event/handler 的完整 Rust schema；本 validator 继续是快速 preflight，runtime schema 由 core 负责。
- 参考 field guide 和 validator note 与 core shape 对齐。

## Breaking surface 检查

- app-server API：无变化。
- raw response items：无变化。
- CLI 参数：无变化。
- config loading：无变化。
- rollout 恢复：无变化。
- Plugin validator：放宽一个 core 已支持字段，同时对该字段执行 preflight；不改变 core wire/runtime。

总 diff 214 additions、3 deletions，3 个相关文件，低于复杂变更 500 行边界。
