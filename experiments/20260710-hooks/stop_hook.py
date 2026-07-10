#!/usr/bin/env python3
"""Validate a Codex Stop-hook payload without writing to disk or the network."""

import json
import sys


def main() -> int:
    try:
        payload = json.load(sys.stdin)
    except (json.JSONDecodeError, OSError) as error:
        print(f"invalid JSON input: {error}", file=sys.stderr)
        return 2

    if payload.get("hook_event_name") != "Stop":
        print("expected hook_event_name=Stop", file=sys.stderr)
        return 2

    required = {"cwd", "session_id", "turn_id", "model", "permission_mode"}
    missing = sorted(required.difference(payload))
    if missing:
        print(f"missing required fields: {', '.join(missing)}", file=sys.stderr)
        return 2

    print(json.dumps({"continue": True, "systemMessage": "read-only study Stop hook observed"}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
