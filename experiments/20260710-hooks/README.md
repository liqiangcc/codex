# Read-only Stop hook experiment

This experiment isolates a Codex `Stop` command hook. The handler reads one
bounded JSON object from standard input, validates the event and required
fields, emits one bounded JSON response, and performs no file or network I/O.

## Direct behavior check

Run the success path:

```text
python3 stop_hook.py < valid-input.json
```

Expected exit code: `0`. Expected output contains `"continue": true` and the
redacted study message.

Run the failure path:

```text
python3 stop_hook.py < invalid-input.json
```

Expected exit code: `2`. Expected stderr is
`expected hook_event_name=Stop`.

## Codex boundary

`hooks.json` uses the same `{ "hooks": { "Stop": [...] } }` shape parsed by
`codex_config::HooksFile`. To inspect it in a live project, place or link it as
`<project>/.codex/hooks.json`, start a new Codex thread, and open `/hooks`.
Codex lists the command as needing review because command hooks run outside the
sandbox. Do not bypass trust for this study. Remove the project hook file to
uninstall it.

The runtime implementation is covered by the `codex-hooks` crate tests; this
experiment separately checks the exact handler input/output and failure exit.
