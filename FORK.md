# Call Desk macOS fork

This fork of [Vercel Labs agent-desktop](https://github.com/vercel-labs/agent-desktop) carries the macOS native-window repairs used by [Shared Call Desk](https://github.com/joewilsonai/shared-call-desk).

Base revision: `5261b5287b5208e7c4bf3713df6f66faaaf2c4cc`.

## Changes

- Discover real application windows from AXWindows, AXMainWindow, AXFocusedWindow and AXChildren.
- Require a unique matching native window ID for explicit window selection. A same-title window is no longer a fallback for a missing native ID.
- Share the corrected resolver across the relevant tree readers and window operations.
- Include regression tests for window discovery and exact-ID matching.

Modified Rust files carry a change notice. [LICENSE](LICENSE) retains the upstream Apache License 2.0. The source revision is the fork identity; the CLI's upstream package version alone does not distinguish this build.

## Install from source

Requirements: macOS, Rust 1.89+ and Xcode command-line tools. The published binary target, if attached to a fork release, is macOS ARM64. Other platforms are not verified for these macOS-specific modifications.

```sh
git clone https://github.com/joewilsonai/call-desk-agent-desktop.git
cd call-desk-agent-desktop
git checkout --detach call-desk-macos-2026-11-02
env -u AGENT_DESKTOP_HOME cargo test --locked \
  -p agent-desktop-core -p agent-desktop-macos -p agent-desktop -p agent-desktop-ffi --lib
env -u AGENT_DESKTOP_HOME cargo build --release --locked -p agent-desktop
env -u AGENT_DESKTOP_HOME cargo build --release --locked -p agent-desktop-macos --bin agent-desktop-macos-helper
./target/release/agent-desktop --version
```

Use the [Call Desk dependency installation guide](https://github.com/joewilsonai/shared-call-desk/blob/main/third_party/agent-desktop/README.md) to preserve an existing tool and install the fork build into a versioned local directory. Native permissions still need verification on the target Mac.

The upstream npm package is a separate distribution. Installing it does not prove that this fork's modifications are present. Do not publish this fork under the upstream npm package name.

## Validation

The release's validation report records which host checks and build completed. The original local patch had passed host library tests, formatting, Clippy and a release build before publication. Publication checks rerun the modified source on macOS. Neither offline tests nor a compiled binary establish complete browser automation or unattended reliability.

This fork does not change Call Desk's message receipts, Google Voice service behavior, or macOS permission requirements.
