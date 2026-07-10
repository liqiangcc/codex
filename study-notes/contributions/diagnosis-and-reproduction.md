# 问题诊断与复现

## 实际行为

最新 upstream 中，同一个 bundled `plugin-creator` 给出互相冲突的行为：

- `references/plugin-json-spec.md` 的示例和 field guide 声明 `"hooks": "./hooks.json"`。
- `codex-rs/core-plugins/src/manifest.rs::RawPluginManifest` 和 `resolve_manifest_hooks` 接受 path、path array、inline object 和 inline object array。
- `scripts/validate_plugin.py::validate_manifest_shape` 的 `allowed_keys` 不含 `hooks`，所以 Skill 指示用户运行的 validator 拒绝文档自身推荐的 manifest。
- 同一参考文件末尾又写“validator rejects ... hooks”，与开头示例和当前 core loader 相反。

这不是账号、配置或环境问题：失败发生在纯本地 JSON 预检，且与 core loader 的已存在类型直接冲突。

## 最小复现

测试先于修复加入，构造一个满足其他必填字段的 plugin：

```json
{
  "name": "hooks-plugin",
  "version": "1.0.0",
  "description": "Plugin with lifecycle hooks.",
  "author": {"name": "Test Author"},
  "hooks": "./hooks.json",
  "interface": {
    "displayName": "Hooks Plugin",
    "shortDescription": "Validate plugin hooks",
    "longDescription": "Validate lifecycle hook declarations.",
    "developerName": "Test Author",
    "category": "Productivity",
    "capabilities": ["Hooks"],
    "defaultPrompt": "Run the hooks plugin."
  }
}
```

修复前运行 `python3 -m unittest -v test_validate_plugin.py`：2/2 失败。关键实际错误是：

```text
plugin.json field `hooks` is not accepted by plugin validation
```

期望：存在且结构合法的 `./hooks.json` 通过；缺失文件返回针对文件的错误，而不是把受支持字段视为未知字段。

## 分类

结论：bundled developer tool bug，并伴随参考资料漂移。它会阻断已被 core 支持、且被 Skill 示例推荐的 Plugin Hook 配置。
