use std::sync::Arc;

use anyhow::Result;
use channel_versions::Changelog;

use crate::server::server_api::ServerApi;

/// PrompTTY has no hosted changelog artifacts, so this always returns `None`.
pub async fn get_current_changelog(_server_api: Arc<ServerApi>) -> Result<Option<Changelog>> {
    Ok(None)
}
