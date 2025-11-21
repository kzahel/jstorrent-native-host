# JSTorrent Native Host

A minimal, cross-platform Native Messaging Host for the JSTorrent Chrome Extension, written in Rust.

## Overview

This binary acts as a bridge between the JSTorrent Chrome extension and the host operating system, providing safe access to:
- TCP/UDP Sockets
- File System Operations (confined to a download root)
- Atomic File Moves
- Native Folder Picker
- SHA1 Hashing

It is designed to be a "dumb" proxy with no BitTorrent logic, strictly enforcing security policies.

## Documentation

- [Design Document](design.md): Detailed architecture and requirements.
- [Installer Design](installer-design.md): Packaging and distribution strategy.
- [Vibe Artifacts](vibe_artifacts/): Development history and artifacts.

## Build & Install

### Prerequisites
- Rust (stable)

### Build
```bash
cargo build --release
```

### Install

#### Linux (One-Line Install)
Run the following command in your terminal:
```bash
curl -fsSL https://kyle.graehl.org/jstorrent-native-host/install.sh | bash
```
*(Note: Requires `curl` and `bash`)*

#### Windows & macOS
1.  Go to the [Releases Page](https://github.com/kzahel/jstorrent-native-host/releases/latest).
2.  Download the installer for your platform:
    - **Windows**: `jstorrent-native-host-install-windows-x86_64.exe`
    - **macOS**: `jstorrent-native-host-install-macos-x86_64.pkg`
3.  Run the installer.

## Magnet Link Handling

The host supports handling `magnet:` links via a separate stub binary (`jstorrent-magnet-stub`).

### Architecture
1.  **Stub Binary**: Registered as the OS handler for `magnet:` scheme.
2.  **Discovery**: The stub finds running host instances by looking for `rpc-info-*.json` files in the config directory.
3.  **RPC**: The stub communicates with the host via a local HTTP server (port and token found in discovery file).
4.  **Fallback**: If no host is running, the stub launches the browser to handle the link (via the extension).

## Development & Testing

### Running Integration Tests

To verify the magnet handler integration:

1.  Build the project:
    ```bash
    cargo build
    ```
2.  Run the verification script:
    ```bash
    python3 verify_magnet.py
    ```

This script starts the host, waits for initialization, runs the stub with a magnet link, and verifies that the host receives the `MagnetAdded` event.
