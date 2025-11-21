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

## Link & File Handling

The host supports handling both `magnet:` links and `.torrent` files via a unified stub binary (`jstorrent-link-handler`).

### Architecture
1.  **Stub Binary**: Registered as the OS handler for `magnet:` scheme and `.torrent` file extension.
2.  **Discovery**: The stub finds running host instances by looking for `rpc-info-*.json` files in the config directory.
3.  **RPC**: The stub communicates with the host via a local HTTP server (port and token found in discovery file).
    - For magnets: `POST /add-magnet`
    - For torrents: `POST /add-torrent` (sends file contents)
4.  **Fallback**: If no host is running, the stub launches the browser to handle the link/file (via the extension).

## Development & Testing

### Running Integration Tests

To verify the integration:

1.  Build the project:
    ```bash
    cargo build
    ```
2.  Run the verification scripts:
    ```bash
    # Verify magnet link handling
    python3 verify_magnet.py

    # Verify .torrent file handling
    python3 verify_torrent.py
    ```

These scripts start the host, wait for initialization, run the stub with a magnet link or torrent file, and verify that the host receives the corresponding event (`MagnetAdded` or `TorrentAdded`).
