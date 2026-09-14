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

## Real MSVC icon-group fixture

`msvc-group-icon.bin` is the 90-byte RT_GROUP_ICON entry extracted with pefile
from the Windows CI executable in run `34846769892`, commit
`4bf98cc649c5fd92361db49e3d113f327110c3d4` (not a generated mock).

- Executable SHA-256: `ef6bb535dd54473be5c581b4a06923291d764e38cd5f4f69d57b246a925a8a61`
- Fixture SHA-256: `cfdbc7813628f637d2c54ab7b54b6c6d4a7ba59a7805a118315b1f915a72dddb`

All six embedded PNG resources matched the approved ICO payloads exactly.
MSVC changed only the directory color-plane field from unspecified `0` to `1`.
The regression permits that PNG metadata normalization while rejecting changed
pixels/encoded image bytes, dimensions, bit depth, or any other plane value.
