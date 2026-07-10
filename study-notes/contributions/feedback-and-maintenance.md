# 反馈与维护

## 模拟 review feedback

初次实现有正常/缺失文件测试，但新增路径 resolver 属于安全边界，缺少两类失败回归：

> 请锁定 plugin archive 逃逸和损坏 Hook JSON；否则后续路径重构可能放松边界，validator 也可能把不可解析文件误报为已支持。

## 响应

新增：

- `test_rejects_hooks_file_outside_plugin`：`./../hooks.json` 必须得到 archive boundary 错误。
- `test_rejects_invalid_hooks_json`：损坏 JSON 必须得到明确 parse 错误。

反馈后结果：Python 7/7、`codex-skills` 1/1、scoped Clippy、格式化全部通过。反馈没有扩大产品功能，只补强新安全边界。

## CI 与冲突处理策略

本阶段没有真实 upstream PR，因此没有虚构 CI 状态。若获邀请并打开 PR：

1. 先 fetch upstream，确认提交仍可复现。
2. 对真实 CI 失败先读日志，区分环境、flake 和确定性代码失败。
3. 只重试疑似 flake；确定性失败在贡献分支做小提交修正。
4. 不用 reset/checkout 丢弃冲突；理解双方 diff 后手工解决并重跑目标测试。
5. 对 review thread 逐条回复证据，变更后再标记 resolved。
6. 合并后更新学习笔记中的最终状态；若方案被拒绝，记录原因而不是隐藏。

## 后续维护触发器

- `RawPluginManifestHooks` shape 改变。
- HooksFile 顶层 schema 改变。
- Plugin validator 被迁移到共享 schema。
- bundled Skill 不再分发 Python validator。

触发后应同时更新 validator、测试和参考资料，避免本次漂移重现。
