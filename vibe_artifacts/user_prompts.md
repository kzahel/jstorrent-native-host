# User Prompts & Actions

This document records the user prompts provided during the development of the JSTorrent Native Host and the corresponding actions taken.

## 1. Initial Host Implementation
**Prompt:** Develop a minimal, cross-platform syscall proxy in Rust for a Chrome extension...
**Actions:**
- Created Rust project structure.
- Implemented IPC, TCP/UDP, File I/O, Path Safety, Atomic Move, Folder Picker, Hashing.
- **Artifacts:** `01_tasks_host.md`, `01_implementation_plan_host.md`, `01_walkthrough_host.md`.

## 2. Installer Implementation
**Prompt:** Create install packages for the JSTorrent Native Host binary across Windows, macOS, and Linux...
**Actions:**
- Created Inno Setup script (Windows), pkg scripts (macOS), shell scripts (Linux).
- Created GitHub Actions workflow.
- **Artifacts:** `02_tasks_installers.md`, `02_implementation_plan_installers.md`.

## 3. CI Fixes (Artifact Upload Deprecation)
**Prompt:** the builds for all 3 platforms failed with the same error: This request has been automatically failed because it uses a deprecated version of `actions/upload-artifact: v3`...
**Actions:**
- Upgraded `actions/upload-artifact` and `actions/checkout` to v4.

## 4. Documentation (README)
**Prompt:** please add a README to the repository but keep it fairly brief and link to other documents...
**Actions:**
- Created `README.md`.

## 5. Linux Build Fix (Dependencies)
**Prompt:** the linux build fails with this error... The system library `glib-2.0` required by crate `glib-sys` was not found...
**Actions:**
- Added `libgtk-3-dev` installation to Linux CI job.

## 6. CI Fixes (Set-Output Deprecation)
**Prompt:** there is also a repeated warning for the linux build: The `set-output` command is deprecated...
**Actions:**
- Replaced `actions-rs/toolchain` with `dtolnay/rust-toolchain`.

## 7. Windows Build Fix (PowerShell)
**Prompt:** the windows compiler installer step fails with this message... Unexpected token 'installers\windows\jstorrent.iss'...
**Actions:**
- Added `&` operator to Inno Setup command in CI.

## 8. macOS Build Fix (Binary Name)
**Prompt:** the macos prepare pkg root step fails... cp: target/release/jstorrent-native-host: No such file or directory...
**Actions:**
- Corrected binary name mismatch (`jstorrent-host` vs `jstorrent-native-host`) in CI and Windows installer.

## 9. Windows Installer Fix (Type Mismatch)
**Prompt:** for the windows installer, under compile installer, i see this error... Type mismatch...
**Actions:**
- Fixed `jstorrent.iss` to use `AnsiString` for `LoadStringFromFile`.

## 10. Release Management
**Prompt:** all the builds are succeeding now... i would like to provide links to the installers from the releases section... please provide me with instructions...
**Actions:**
- Created `release-management.md`.
- Updated CI to trigger on tags and upload release assets.

## 11. Release Permissions
**Prompt:** i created a tag (v0.0.1) and pushed it... GitHub release failed with status: 403...
**Actions:**
- Added `permissions: contents: write` to CI workflow.

## 12. One-Line Installer
**Prompt:** i would like to provide linux users with a command they can paste into their terminal... `curl .. | bash`...
**Actions:**
- Created `docs/install.sh`.
- Updated `README.md` with install instructions.

## 13. GitHub Pages
**Prompt:** can you please setup github pages for this repository... also create a small html page...
**Actions:**
- Created `docs/index.html`.
- Created `deploy-pages.yml` workflow.

## 14. Artifact Archival
**Prompt:** please add further implementation notes... into the vibe_artifacts folder...
**Actions:**
- Archived internal artifacts to `vibe_artifacts/` with sequential naming.
- Created `03_summary_ci_release.md`.
