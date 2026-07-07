# Codex 学习笔记

`00-study/codex-agent-learning` 分支用于学习 `openai/codex` 的源码和 agent 运行机制。`main` 分支保持紧跟上游，本分支保存笔记、实验和阅读路线，并用 `00-` 前缀让它在分支列表中更靠前。

## 学习约定

- [学习目标和原则](goal-and-principles.md)
- [第一阶段：用好 Codex CLI](stage-1-use-codex-well.md)
- [Codex CLI 功能地图](feature-map.md)
- [Codex CLI 详细功能文档](features/)
- [学习体系维护手册](maintenance.md)
- [学习检查清单](checks.md)
- [Feature Study issue 模板](issue-templates/feature-study.md)
- [学习方法探索](learning-method.md)
- [Codex 架构阅读路线](codex-architecture.md)
- [Agent 机制笔记](agent-notes.md)

## 推荐学习顺序

### 阶段 1：用好 Codex CLI

1. 读 [学习目标和原则](goal-and-principles.md)，确认学习边界。
2. 读 [第一阶段：用好 Codex CLI](stage-1-use-codex-well.md)，按闭环推进。
3. 从 [Codex CLI 功能地图](feature-map.md) 选择一个功能。
4. 阅读对应 [详细功能文档](features/)，先理解背景、边界、入口、练习和实验设计。
5. 打开对应 GitHub issue，用手机记录问题、观察和下一步。
6. 做最小实验或真实任务，把稳定结论整理回详细功能文档。
7. 每次学习后按 [学习检查清单](checks.md) 做收尾检查。

### 阶段 2：读源码实现

阶段 1 跑通后，再进入实现层阅读：

1. 先读 `README.md` 和 `docs/getting-started.md`，确认 Codex CLI 的用户模型。
2. 读 `docs/config.md`、`docs/sandbox.md`、`docs/exec.md`，理解配置、权限和执行边界。
3. 读 `codex-cli/bin/codex.js` 和 `codex-rs/cli/`，跟踪命令如何进入 Rust CLI。
4. 读 `codex-rs/core/`，梳理 agent loop、会话状态、模型请求、工具调用和上下文压缩。
5. 结合 `codex-rs/exec/`、`codex-rs/exec-server/`、`codex-rs/hooks/` 理解工具执行和扩展点。

## 仓库入口

- `README.md`：产品定位、安装方式和官方文档入口。
- `docs/`：面向用户的配置、认证、沙箱、slash commands、AGENTS.md 等文档。
- `codex-cli/`：npm 包装层和命令入口。
- `codex-rs/cli/`：Rust CLI 入口。
- `codex-rs/core/`：核心 agent 逻辑的主要阅读入口。
- `codex-rs/exec/` 与 `codex-rs/exec-server/`：命令执行、隔离执行和工具运行相关逻辑。
- `codex-rs/codex-mcp/`：MCP 相关能力。

## 同步上游

同步上游的唯一维护流程见 [学习体系维护手册](maintenance.md#每次学习前)，避免多处命令漂移。
