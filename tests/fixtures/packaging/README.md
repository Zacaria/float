# Stale release icon fixtures

These are verbatim source icons from shipped v0.2.0 commit `e6b27c8`, extracted
with `git show e6b27c8:src-tauri/icons/icon.<extension>`. They are negative test
inputs only, never bundle inputs or approved branding exports.

- `v0.2.0-icon.icns`: SHA-256 `d2c591defc5961f2e5c75e667c95bc645867ab4ac22ad3bea45b724e4a8a6e14`
- `v0.2.0-icon.ico`: SHA-256 `eccdc3b4a07185a06724c3617ae92f558b888778583a365e84585a168be74b25`

The tests place the old ICNS in a temporary app bundle and encode the old ICO
frames in a minimal PE resource section. pefile parses those PE bytes through
the same reader used for release verification. The fixtures are not runnable
Windows programs; their tests do not establish native installation or signing.
