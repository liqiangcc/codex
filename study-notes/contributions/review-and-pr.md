# Review 与 PR

## 本地 review

Review 范围限定为 `upstream/main..53a917a3ac`：

- 行为是否与 `RawPluginManifestHooks` 四种 shape 一致。
- 路径是否要求 `./`、拒绝 `..` 并限制在 plugin root。
- 是否对 missing/invalid JSON 给出可操作错误。
- 是否避免复制完整 Hook schema。
- 文档、validator 和测试是否描述同一 contract。
- 是否触及 breaking surfaces、secret、网络或账号行为。

最终没有未解决的 P0/P1/P2 发现。已处理的一条 review feedback 见 [反馈与维护](feedback-and-maintenance.md)。

## 模拟 PR

标题：`Accept plugin hooks in bundled validator`

正文保存在 [draft-pr.md](draft-pr.md)，包含 What/Why/How、复现、测试、风险和政策说明。

## 为什么没有向 upstream 创建 PR

`docs/contributing.md` 当前明确写明外部代码贡献仅限邀请，未经邀请的 PR 会直接关闭。为遵守这一政策：

- 没有向 `openai/codex` 创建 PR 或 issue 噪音。
- 已把原子提交推送到个人 fork 的贡献分支，保留可审阅 diff。
- 用本地 draft PR 包完成描述和 review 演练。
- 如果维护者未来邀请，可从最新 main 重新验证后提交，而不是直接复用过时结论。

## 分支状态

- original base：`6138909d6e`（创建分支时最新 upstream/main）
- current base：`2b9c050460`（最终验收时最新 upstream/main）
- fix commit：`53a917a3ac`
- refreshed head：`d7d109c3f6`
- remote：`origin/agent/plugin-creator-hooks-validation`
- working tree：提交后 clean
