# Permissions Sandbox Experiment

## Purpose

Compare the same bounded write request under `read-only` and `workspace-write`.

## Commands

```text
codex --ask-for-approval never exec --sandbox read-only ...
codex --ask-for-approval never exec --sandbox workspace-write ...
```

Both prompts prohibited network access, Git commands, and changes outside this
experiment path.

## Observation

- `read-only` refused to create the probe file; the process exited successfully and the file did not exist.
- `workspace-write` created only `write-probe.txt` with `workspace-write verified`.

## Conclusion

The sandbox constrains actual tool execution even when the task prompt requests
a write. Further external-path and network tests require a dedicated safe
environment and are intentionally not part of this repository experiment.
