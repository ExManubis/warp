//! Release-channel `warp-tui` binary and `default-run` target.

use anyhow::Result;
use warp_core::AppId;
use warp_core::channel::{Channel, ChannelConfig, ChannelState};

fn main() -> Result<()> {
    let mut state = ChannelState::new(
        Channel::Release,
        ChannelConfig::local_only(
            AppId::new("dev", "promptty", "PrompTTYTui"),
            "promptty-tui.log",
        ),
    );
    if cfg!(debug_assertions) {
        state = state.with_additional_features(warp_core::features::DEBUG_FLAGS);
    }
    ChannelState::set(state);

    warp_tui::run()
}
