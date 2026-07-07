# Codex 架构阅读路线

## 第一层：产品和入口

Codex CLI 是一个运行在本机终端里的 coding agent。仓库顶层 `README.md` 给出安装和使用方式，`docs/` 给出用户可见的配置与行为说明。

优先关注：

- `codex-cli/bin/codex.js`：npm 安装后的 JavaScript 启动层。
- `codex-rs/cli/`：Rust CLI 参数解析和命令分发。
- `docs/config.md`：模型、权限、工具和运行参数的配置面。

## 第二层：核心 agent

`codex-rs/core/` 是理解 agent 的重点。阅读时建议按下面问题做索引：

- 一次用户输入如何转成模型请求？
- 模型返回的 tool call 如何被调度？
- 工具执行结果如何回填到下一轮上下文？
- 会话状态、历史消息和上下文压缩在哪里维护？
- 权限、沙箱、approval policy 如何影响执行路径？

## 第三层：工具执行和隔离

重点目录：

- `codex-rs/exec/`：本地命令执行能力。
- `codex-rs/exec-server/`：执行服务和长生命周期执行环境。
- `codex-rs/execpolicy/`：执行策略。
- `codex-rs/hooks/`：运行时 hook 和扩展点。
- `codex-rs/codex-mcp/`：MCP 工具和外部能力接入。

## 第四层：可观测和状态

后续再读：

- `codex-rs/rollout-trace/`：对话、工具和执行轨迹建模。
- `codex-rs/analytics/`：事件和指标采集。
- `codex-rs/app-server/`：如果需要理解桌面或 app server 场景，再深入阅读。

