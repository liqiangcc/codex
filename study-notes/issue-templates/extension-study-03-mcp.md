## 目标

验证一个只读 MCP 的启动、调用、错误、权限和清理边界。

## 完成证据

- 子 Codex 固定为 `approval=never`、`sandbox=read-only`。
- `/skills` 观察成功，未知 session 得到确定性失败。
- finally 分支 stop 成功。
- 输入、等待、屏幕和 transcript 均有硬上限与 redaction。

详细记录：[MCP 工具与权限](https://github.com/liqiangcc/codex/blob/00-study/codex-agent-learning/study-notes/extensions/mcp-tools-and-permissions.md)

## 结论

server 不接受 arbitrary executable/arguments，MCP 提供的是最小 PTY 能力，不是通用远程 shell。
