# Agent 机制笔记

## 需要重点理解的问题

- Agent loop：用户消息、模型响应、工具调用、工具结果、下一轮模型请求之间如何循环。
- Tool calling：模型如何表达工具调用，Codex 如何校验、执行并返回结果。
- Context management：系统消息、开发者消息、用户消息、仓库上下文、工具结果如何组合。
- Compaction：上下文过长时如何摘要或压缩。
- Permission model：不同 approval policy、sandbox、network setting 对工具执行的影响。
- Git workflow：Codex 如何读取状态、避免覆盖用户改动、生成 patch、运行测试。

## 阅读记录模板

每读一个模块，用下面格式记录：

```markdown
## 模块

- 路径：
- 作用：
- 关键类型/函数：
- 输入：
- 输出：
- 与 agent loop 的关系：
- 还没理解的问题：
```

## 初始假设

- Codex 的核心不是单次调用模型，而是一个带状态的循环：模型提出操作，运行时执行操作，再把结果喂回模型。
- 工具执行安全性由多层共同控制：prompt 约束、approval policy、sandbox、exec policy、Git dirty-state 检查。
- 学习时应优先追踪一条完整链路，而不是先横向阅读所有 crate。

