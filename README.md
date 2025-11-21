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
    - **Windows**: `jstorrent-installer.exe`
    - **macOS**: `jstorrent-native-host.pkg`
3.  Run the installer.
