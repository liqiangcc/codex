# 最小实现与验证

## 变更

贡献分支：`agent/plugin-creator-hooks-validation`

提交：`53a917a3ac Accept plugin hooks in bundled validator`

文件：

- `plugin-creator/scripts/validate_plugin.py`：接受并预检 Hook declarations。
- `plugin-creator/scripts/test_validate_plugin.py`：独立 Python 行为测试。
- `plugin-creator/references/plugin-json-spec.md`：移除自相矛盾说明，记录四种受支持 shape。

## 测试顺序与结果

1. 修复前 Python 测试：2/2 按预期失败，证明测试能捕获原 bug。
2. 初次修复后 Python 测试：5/5 通过。
3. Review feedback 后 Python 测试：7/7 通过，新增 archive escape 和 invalid JSON。
4. `just test -p codex-skills`：1/1 通过。
5. `just fix -p codex-skills`：通过，无 Clippy 警告。
6. `just fmt`：通过。

测试使用仓库固定 Rust 1.95.0、`cargo-nextest`、新版 `just`、`dotslash` 和 `uv`。首次构建缺少 `pkg-config`/`libssl-dev`，补齐系统依赖后原命令通过；这被归类为环境依赖，不是断言失败。

按仓库约定，最终 `fix`/`fmt` 后未再次运行测试；测试在它们之前已通过。

## 未覆盖风险

- Python preflight 不复制每个 Hook event/handler 的完整 Rust schema；这是有意控制重复实现。
- 测试文件随 bundled Skill 资源分发，但不在正常 Skill 流程中自动执行；它提供可手动复现的独立回归入口。
- 未运行完整 workspace 测试：变更只在 `codex-skills` 资源和 Python validator 内，不涉及 common/core/protocol。
