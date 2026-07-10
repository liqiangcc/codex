# 可重复贡献检查清单

## 开始前

- [ ] 阅读最新根 `AGENTS.md`、相关目录 guidance 和 `docs/contributing.md`。
- [ ] fetch upstream 并记录 base SHA；确认 fork main 是否能纯 fast-forward。
- [ ] 从 upstream main 创建单问题 branch 和独立 worktree。
- [ ] 检查工作区没有用户未提交改动。

## 诊断

- [ ] 写出最小实际/期望行为和可复制命令。
- [ ] 用测试或 fixture 先证明失败，不把推断写成事实。
- [ ] 排除配置、版本、环境和缺失信息。
- [ ] 标出关键源码类型、入口和跨层不变量。

## 设计

- [ ] 比较至少两个方案，选择最小完整变更。
- [ ] 检查 app-server、raw events、CLI、配置和 rollout breaking surfaces。
- [ ] 控制 change size；不混入无关重构或文档。
- [ ] 明确权限、数据、平台和回滚边界。

## 实现与验证

- [ ] 先运行失败测试，再实现，再运行目标测试。
- [ ] 使用 `just test -p <crate>`，不直接运行 `cargo test`。
- [ ] 需要时运行 scoped `just fix -p <crate>`，最后运行 `just fmt`。
- [ ] 按仓库顺序，不在最终 fix/fmt 后重跑测试。
- [ ] 记录命令、结果、环境依赖和未覆盖风险。

## Review 与发布

- [ ] review 实际 diff，检查 secrets、路径、错误语义和测试盲区。
- [ ] 准备 What/Why/How、复现、验证和风险齐全的 PR 正文。
- [ ] 遵守上游邀请/贡献政策，不创建未经允许的外部状态。
- [ ] 提交原子 commit，推送明确 remote branch；禁止 force。
- [ ] 处理一轮 feedback，保持修正与原问题相关。
