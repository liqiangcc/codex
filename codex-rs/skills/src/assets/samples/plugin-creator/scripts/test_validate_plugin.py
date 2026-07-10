import json
import tempfile
import unittest
from pathlib import Path

from validate_plugin import validate_plugin


class ValidatePluginHooksTest(unittest.TestCase):
    def setUp(self) -> None:
        self.temp_dir = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp_dir.cleanup)
        self.plugin_root = Path(self.temp_dir.name)
        (self.plugin_root / ".codex-plugin").mkdir()

    def write_manifest(self, hooks: object) -> None:
        manifest = {
            "name": "hooks-plugin",
            "version": "1.0.0",
            "description": "Plugin with lifecycle hooks.",
            "author": {"name": "Test Author"},
            "hooks": hooks,
            "interface": {
                "displayName": "Hooks Plugin",
                "shortDescription": "Validate plugin hooks",
                "longDescription": "Validate lifecycle hook declarations.",
                "developerName": "Test Author",
                "category": "Productivity",
                "capabilities": ["Hooks"],
                "defaultPrompt": "Run the hooks plugin.",
            },
        }
        (self.plugin_root / ".codex-plugin" / "plugin.json").write_text(
            json.dumps(manifest), encoding="utf-8"
        )

    def test_accepts_hooks_file_declared_by_manifest(self) -> None:
        self.write_manifest("./hooks.json")
        (self.plugin_root / "hooks.json").write_text(
            json.dumps(
                {
                    "hooks": {
                        "Stop": [
                            {
                                "hooks": [
                                    {"type": "command", "command": "true"}
                                ]
                            }
                        ]
                    }
                }
            ),
            encoding="utf-8",
        )

        self.assertEqual(validate_plugin(self.plugin_root), [])

    def test_rejects_missing_hooks_file(self) -> None:
        self.write_manifest("./hooks.json")

        self.assertEqual(
            validate_plugin(self.plugin_root),
            ["hooks file `./hooks.json` does not exist"],
        )

    def test_rejects_hooks_file_outside_plugin(self) -> None:
        self.write_manifest("./../hooks.json")

        self.assertEqual(
            validate_plugin(self.plugin_root),
            ["plugin.json field `hooks` paths must stay inside the plugin archive"],
        )

    def test_rejects_invalid_hooks_json(self) -> None:
        self.write_manifest("./hooks.json")
        (self.plugin_root / "hooks.json").write_text("{", encoding="utf-8")

        self.assertEqual(
            validate_plugin(self.plugin_root),
            ["hooks file `./hooks.json` must contain valid JSON"],
        )

    def test_accepts_inline_hook_declarations(self) -> None:
        self.write_manifest(
            {
                "hooks": {
                    "Stop": [
                        {"hooks": [{"type": "command", "command": "true"}]}
                    ]
                }
            }
        )

        self.assertEqual(validate_plugin(self.plugin_root), [])

    def test_accepts_multiple_hooks_files(self) -> None:
        self.write_manifest(["./first.json", "./second.json"])
        for filename in ("first.json", "second.json"):
            (self.plugin_root / filename).write_text(
                json.dumps({"hooks": {}}), encoding="utf-8"
            )

        self.assertEqual(validate_plugin(self.plugin_root), [])

    def test_rejects_mixed_hooks_array(self) -> None:
        self.write_manifest(["./hooks.json", {"hooks": {}}])

        self.assertEqual(
            validate_plugin(self.plugin_root),
            [
                "plugin.json field `hooks` must be a string, string array, object, or object array"
            ],
        )


if __name__ == "__main__":
    unittest.main()
