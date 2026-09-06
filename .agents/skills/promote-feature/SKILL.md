---
name: promote-feature
description: Promote a feature-flagged feature to all PrompTTY users. Use when a feature behind a FeatureFlag is ready to ship, including wiring up the compile-time/runtime bridge and deferring flag cleanup safely.
---

# promote-feature

Guides shipping a gated `FeatureFlag` to PrompTTY Release, and schedules the follow-up cleanup.

## Overview

Feature flags have two interacting layers:
- **Runtime** (`warp_core/src/features.rs`): `DEBUG_FLAGS` is applied on debug builds. There is no dogfood/preview/release channel ladder.
- **Compile-time** (`app/Cargo.toml` + `app/src/lib.rs`): Cargo features in `[features]`. The `default = [...]` array enables a feature for all builds. `enabled_features()` in `app/src/lib.rs` bridges each Cargo feature to its `FeatureFlag` variant via `#[cfg(feature = "...")]`.

**Do not remove the flag immediately after promoting.** Keep it for at least 1–2 release cycles so a rollback is a one-line PR (remove the entry from `default`). Use the `remove-feature-flag` skill for the cleanup step later.

## TUI note

The `FeatureFlag` enum and `FeatureFlag::X.is_enabled()` are shared by the GUI (`app/`) and TUI (`crates/warp_tui`). The Cargo-feature bridge in `app/src/lib.rs` is GUI-app-only. If a promoted feature should also reach the TUI and relies on a compile-time Cargo feature, enable it in `crates/warp_tui/Cargo.toml` too.

## Promote to Release

### 1. `app/Cargo.toml` — add to `default`

```toml
default = [
    # ...
    "your_feature_name",
]
```

### 2. `app/src/lib.rs` — add to `enabled_features()` bridge

```rust
#[cfg(feature = "your_feature_name")]
FeatureFlag::YourFeature,
```

Place it near logically related entries.

### Validate

```bash
./script/format
cargo clippy --workspace --all-targets --all-features --tests -- -D warnings
```

A future PrompTTY Preview channel can reintroduce a `PREVIEW_FLAGS` array on that bin only. Until then, ship via Cargo `default`.
