mod config;
mod state;

use std::fmt;

pub use config::*;
pub use state::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    /// The public PrompTTY product build.
    Release,
    /// The integration test build.
    Integration,
}

impl Channel {
    /// Whether this channel honors the `--server-root-url` / `--ws-server-url` /
    /// `--session-sharing-server-url` flags (and their `WARP_*` env-var equivalents).
    ///
    /// Release ignores these overrides: there is no hosted backend to retarget.
    /// Integration honors them so tests can point at dummy or mock servers.
    pub fn allows_server_url_overrides(&self) -> bool {
        matches!(self, Channel::Integration)
    }

    /// Returns the CLI command name corresponding to this channel.
    pub fn cli_command_name(&self) -> &'static str {
        match self {
            Channel::Release => "promptty",
            Channel::Integration => "promptty-integration",
        }
    }

    /// Returns the Warp Control CLI command name corresponding to this channel.
    pub fn warpctrl_command_name(&self) -> &'static str {
        match self {
            Channel::Release => "prompttyctrl",
            Channel::Integration => "prompttyctrl-integration",
        }
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Channel::Release => "release",
            Channel::Integration => "integration",
        })
    }
}
