# PrompTTY build channels

PrompTTY is a permanent, local-first fork of Warp. Warp’s six-channel model (Stable, Preview, Dev, Local, Oss, Integration) exists to isolate hosted SaaS products and to bake in `warp-channel-config` (servers, telemetry, autoupdate, Sentry). That generator is not in this repo. The only channel that already matches the product is Oss (`ChannelConfig::local_only`).

This spec replaces Warp’s product slots with two PrompTTY identities and keeps the channel *mechanism* so a Preview channel can be added later without rebuilding isolation.

## Decisions (locked)

- One public product plus a test identity (not a Preview/Dev/Local ladder).
- PrompTTY-native names, bundle IDs, and config dirs — not `Channel::Oss` / `.warp-oss` / `dev.warp.WarpOss`.
- Delete Warp’s unused enum variants. Do not keep dead `Stable` / `Preview` / `Dev` / `Local` / `Oss` arms for cherry-picks.
- Keep `Channel` + `ChannelState` + `ChannelConfig`. Do not collapse to `cfg(test)` / a boolean.
- No migration from `~/.warp-oss`. New identity, new dirs.

## In scope

1. Shrink `Channel` to `Release` and `Integration`.
2. Rebrand Release to PrompTTY-native binary names, app IDs, paths, URL schemes, CLI names.
3. Always use `ChannelConfig::local_only` for Release. Integration keeps its dummy black-hole servers.
4. Attach extra feature flags to build type (`debug_assertions` vs Cargo `default`), not to a dogfood channel.
5. Remove `warp-channel-config` / `load_config!` from the run and bundle path.
6. Point `./script/run` and `./script/run-tui` at the PrompTTY binaries only.
7. Update exhaustive `match Channel` sites, channel tests, and agent/docs that describe the Warp ladder.

## Out of scope

- Adding a PrompTTY Preview channel (door stays open; see “Adding Preview later”).
- Replacing Warp autoupdate, telemetry, Drive, billing, or Oz with PrompTTY-hosted services.
- Renaming `warp_*` crates.
- Migrating user data from `.warp` / `.warp-oss`.
- Changing which individual `FeatureFlag` variants exist; only how they are enabled by channel.

## Identities

```rust
pub enum Channel {
    Release,
    Integration,
}
```

`ChannelState` remains process-global: current `Channel`, `ChannelConfig`, and `additional_features`. `ChannelState::init()` (tests and any code that runs before a bin sets state) defaults to `Release` + `local_only`.

`is_dogfood()` is removed. Call sites that branched on dogfood either drop the branch (Release is not dogfood) or use `cfg!(debug_assertions)` when the intent was “developer build.”

`allows_server_url_overrides()` is `true` only for `Integration`. Release ignores `--server-root-url` / `--ws-server-url` / `--session-sharing-server-url` and their existing `WARP_*` env equivalents. This change does not rename those flags.

### Release (the product)

| Surface | Value |
|---|---|
| Enum | `Channel::Release` |
| Display / script channel string | `release` |
| GUI binary / `default-run` | `promptty` |
| TUI binary / `warp_tui` default-run | `promptty-tui` |
| CLI command | `promptty` |
| Warp Control command | `prompttyctrl` |
| Bundle ID | `dev.promptty.PrompTTY` (`AppId::new("dev", "promptty", "PrompTTY")`) |
| GUI config dir | `~/.promptty` (all platforms; do not keep a `.warp*` prefix) |
| TUI config dir | `~/.promptty-tui` (isolated from the GUI; do not share `~/.promptty`) |
| URL scheme | `promptty://` |
| GUI log file | `promptty.log` |
| TUI app ID / log | `dev.promptty.PrompTTYTui` / `promptty-tui.log` |
| `ChannelConfig` | `ChannelConfig::local_only(...)` — no server, Oz, telemetry, autoupdate, crash reporting, or bundled MCP OAuth |

Linux package / desktop ID follows the same identity (`promptty`, `dev.promptty.PrompTTY`), not `warp-terminal` / `warp-oss`.

### Integration (test harness)

| Surface | Value |
|---|---|
| Enum | `Channel::Integration` |
| Display / script channel string | `integration` |
| Binary | `promptty-integration` |
| CLI command | `promptty-integration` |
| Bundle ID | macOS `dev.promptty.PrompTTY-Integration`; other platforms `dev.promptty.PrompTTYIntegration` |
| Config dir | `~/.promptty-integration` |
| URL scheme | `prompttyintegration://` |
| Log file | `promptty-integration.log` |
| Servers | unchanged: `http://192.0.2.0:9` and matching WS URL (IANA TEST-NET + discard) |
| Telemetry / autoupdate / Sentry | none |

Integration continues to honor server URL overrides so tests can retarget the dummy endpoints. Existing `Channel::Integration` special cases (crash recovery, HTTP client, isolation, editor/AI guards) keep an Integration arm.

## Feature flags

Keep the `FeatureFlag` enum and the compile-time Cargo bridge (`app/Cargo.toml` `[features]` + `enabled_features()` / `#[cfg(feature = "...")]`).

Delete these **arrays and the promotion ritual**: `DOGFOOD_FLAGS`, `PREVIEW_FLAGS`, `RELEASE_FLAGS`, `LOCAL_FLAGS`. Do not leave empty shelves.

Enablement:

| Build | Extra runtime flags |
|---|---|
| `promptty` / `promptty-tui` with `debug_assertions` | `DEBUG_FLAGS` (`DebugMode`, `RuntimeFeatureFlags`) |
| same binaries, release / `release_bundle` | none beyond Cargo `default` and `#[cfg(feature = ...)]` |
| `promptty-integration` | none; tests enable what they need |

`init_feature_flags()` still unions extras + compile-time features, then calls `ContextFlag::disable_warp_cloud_flags()` when `cloud_enabled()` is false. Release is always `local_only`, so Warp-cloud flags stay off.

`ChannelState::enable_debug_features()` is `cfg!(debug_assertions)` only.

Flags that today are on only for Warp Dev/Local/Preview (dogfood, preview, and local arrays) stay **off** on PrompTTY Release unless they are already in Cargo `default`. That matches current `./script/run` behavior (Oss never loaded those arrays).

When a flag should ship to all PrompTTY users, add it to Cargo `default` and the `enabled_features()` bridge — not to a channel array.

## Delete vs keep

Keep: `Channel`, `ChannelState`, `ChannelConfig`, `local_only`, isolated paths/app IDs/URL schemes.

Remove:

- Enum variants `Stable`, `Preview`, `Dev`, `Local`, `Oss`
- `app/src/bin/{stable,preview,dev,local,oss}.rs` — replace with `app/src/bin/promptty.rs` (and keep a rewritten integration bin)
- Matching `crates/warp_tui/src/bin/{stable,preview,dev,local,oss}.rs` — replace with `promptty-tui.rs`
- `is_dogfood()`
- `crates/warp_channel_config` and all `load_config!` / `warp-channel-config` PATH probes (GUI build script, TUI build script, `script/run`, `script/run-tui`)
- `app/channels/{stable,preview,dev,local,oss}/` — replace with `app/channels/release/` using PrompTTY desktop/plist IDs
- Bundle script branches that select Warp products (`--channel local|dev|preview|stable|oss`). Remaining legal values: `release` (default) and `integration`

The panic “Internal Warp channel builds are not supported in this fork” goes away because nothing invokes the generator.

Scripts still take a channel string for resources and install layout. That string is `release` or `integration`, not six Warp names. Keep existing script env var names (`WARP_CHANNEL`, `WARP_BIN_NAME`) for this change; they hold the new values (`release`, `promptty`). Renaming those env vars is out of scope.

## Tests and rollout

No user-data migration. `~/.warp-oss` is ignored. GUI Release writes `~/.promptty`; TUI Release writes `~/.promptty-tui`; Integration writes `~/.promptty-integration`.

Must keep working:

- Unit tests that never set `ChannelState` see `Release` + `local_only`
- Integration black-hole behavior and Integration-only code paths
- `./script/run` → `promptty`; `./script/run-tui` → `promptty-tui`; neither looks for `warp-channel-config`
- `cloud_enabled() == false` on Release

Delete or rewrite, do not port:

- Preview→Stable config migration tests
- Autoupdate / Linux repo / changelog tests that enumerate Warp channels
- `generate_default_settings.rs` `stable` / `preview` / `dev` tables
- Completer tests that loop `Stable | Preview | Dev`
- `promote-feature` / `add-feature-flag` / `remove-feature-flag` skill text that describes dogfood → preview → `RELEASE_FLAGS`
- AGENTS.md channel-ladder snippet

Verification:

- Workspace compile and `cargo clippy --workspace --all-targets --all-features --tests -- -D warnings` after the enum shrinks (exhaustive matches are the leftover-arm checklist)
- Targeted nextest for `warp_core` (paths/channel), `warp_tui`, and crates that matched on `Channel`
- One real `./script/run` and `./script/run-tui` confirming new binary names and no generator lookup

## Adding Preview later

Not in this change. When needed:

1. Add `Channel::Preview`.
2. Add GUI/TUI bin wrappers that set that channel and a distinct `ChannelConfig` (still `local_only` unless PrompTTY has its own backend by then).
3. Isolate identity: `~/.promptty-preview`, `dev.promptty.PrompTTYPreview`, `prompttypreview://`, CLI `promptty-preview`.
4. Optionally introduce `PREVIEW_FLAGS` on that bin only.

No need to reintroduce `warp-channel-config` or Warp’s Dev/Local/Stable slots.

## Error handling

- Release never depends on an external config binary. Missing `warp-channel-config` is not an error.
- Integration server URLs stay unroutable by default; override only via the Integration override path.
- Invalid override URLs on Integration still print to stderr and continue (current `run()` behavior).
- Release silently ignores server URL override flags.

## Implementation note

This is one mechanical refactor, compile-driven by exhaustive `match Channel`. Do not rename `warp_*` crates in the same change. Do not enable former dogfood/preview flags “while we are here.”
