# Draft PR：Accept plugin hooks in bundled validator

## What

Teach the bundled plugin-creator validator to accept the same plugin `hooks` shapes as the core manifest loader: a path, path list, inline object, or inline object list. Validate referenced files, archive boundaries, JSON shape, and update the bundled reference. Add focused Python regression tests.

## Why

The plugin-creator reference recommends `"hooks": "./hooks.json"`, and core already loads it, but the bundled validator rejects `hooks` as an unknown manifest field. Users following the Skill therefore cannot validate a supported plugin.

## How

- Add `hooks` to the manifest allowlist.
- Dispatch the four supported shapes without duplicating the full Rust event schema.
- Require referenced paths to start with `./`, remain in the plugin archive, exist, and contain valid JSON.
- Validate the HooksFile top-level `description`/`hooks` structure.
- Align the field guide and validation notes.

## Tests

- `python3 -m unittest -v test_validate_plugin.py` — 7 passed.
- `just test -p codex-skills` — 1 passed.
- `just fix -p codex-skills` — passed.
- `just fmt` — passed.

The new tests cover path, path list, inline object, missing file, mixed arrays, archive escape, and invalid JSON.

## Risk

Low. This relaxes the bundled preflight validator for a field already supported by core, while adding file/path checks. It does not change app-server APIs, raw events, CLI arguments, config loading, or rollout compatibility.

## Policy

This is a local draft package only. Do not open it against `openai/codex` without an explicit maintainer invitation under the current contribution policy.
