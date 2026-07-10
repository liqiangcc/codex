## 目标

验证 personal Plugin 的 manifest、安装、升级、cache、禁用和卸载边界。

## 完成证据

- Manifest 同时声明 Skill root 和 MCP server。
- 版本更新为单一 cachebuster 并从 personal marketplace 重装。
- cache 内容与 `codex plugin list` 的 installed/enabled 版本一致。
- 全新线程发现新 Skill，证明 source 与 cache 已同步。

详细记录：[Plugin 分发](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/extensions/plugin-distribution.md)

## 结论

Plugin 是组件分发和版本容器，不提升组件本身权限；disabled plugin 不应产生 effective contributions。
