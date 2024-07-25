//! Contains the [RollupNode] trait.

use crate::SyncStatus;
use anyhow::Result;
use async_trait::async_trait;

/// A provider for the Rollup Node.
#[async_trait]
pub trait RollupNode {
    /// Fetches the sync status of the node.
    async fn sync_status(&self) -> Result<SyncStatus>;
}
