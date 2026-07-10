# MCP 工具与权限

## 运行链路

```text
.mcp.json stdio declaration
  -> plugin loader merges active MCP servers
  -> Session computes runtime MCP config
  -> connection manager initializes server and tools
  -> step runtime dispatches one validated tool call
  -> bounded/redacted result returns to the model
```

## 源码证据

- `codex-rs/core-plugins/src/loader.rs::load_plugin_mcp_servers` 与 `load_plugin_mcp_servers_from_manifest` 解析插件声明。
- `codex-rs/plugin/src/load_outcome.rs::effective_mcp_servers` 只合并 active plugin 的 server 配置。
- `codex-rs/core/src/session/mcp.rs::runtime_mcp_config` 计算当前 session 的配置，`mcp_runtime_for_step` 形成 step 运行时，`refresh_mcp_servers_now` 控制刷新。
- 连接状态与工具变更集中在 `codex-rs/codex-mcp` 的 connection manager 模块；本实验没有把 server 变更逻辑穿透到多层调用。

## Server 设计

`/root/plugins/codex-tui-observer/server.py` 只暴露五个工具：

1. `codex_tui_start`：固定启动 `codex --ask-for-approval never --sandbox read-only`。
2. `codex_tui_send`：只发送最多 4096 bytes 的文本或四个固定 key。
3. `codex_tui_wait_for`：等待最多 30 秒。
4. `codex_tui_screen`：返回最多 256000 bytes 的截断、redacted 屏幕/转录。
5. `codex_tui_stop`：终止进程组并关闭 PTY。

调用者不能选择 executable、参数、sandbox 或 approval policy，也没有通用 shell 工具。常见 email、token 和 credential 形态在返回前 redaction。

## 行为实验

使用同一 `Observer` 生命周期进行验证：

- `start(cwd=/root/codex)` 返回 `sandbox=read-only`。
- 等待初始 `OpenAI Codex` 成功。
- 输入 `/skills` 并进入列表，观察到阶段三 Skill。
- 用 `missing-session` 调用 screen，得到预期错误 `session_id does not identify an active session`。
- `stop` 返回 `stopped=true`，finally 分支保证失败后也清理。

最初仅打开 `/skills` 一级菜单时未找到显示名，这是 UI 状态不匹配而非 MCP 失败；第二次实验等待二级菜单并确认进入列表后成功。

## 权限、数据和失败边界

- 子 Codex 无写权限且从不请求 interactive approval。
- server 自身只管理本机 PTY，不访问网络或外部 API。
- 工具 schema 拒绝未知字段、无效尺寸、无效 key 和双重 text/key。
- wait 超时返回 `found=false`，不会无限阻塞。
- `atexit` 和显式 `stop` 都执行清理；实验仍要求显式 stop 以便获得证据。
- 不保存完整 screen/transcript、账户信息和 session ID。
