//! Batch Submitter

use anyhow::Result;
use crate::traits::{RollupNode, L2Provider};

/// The Batch Submitter.
///
/// Drives the batch submission process.
#[derive(Debug, Clone)]
pub struct BatchSubmitter<R, L2P>
where
    R: RollupNode + Clone + Debug,
    L2P: L2Provider + Clone + Debug,
{
    /// Holds the block cursor.
    cursor: BlockCursor<R, L2P>
}

impl<R, L2P> BatchSubmitter<R, L2P>
where
    R: RollupNode + Clone + Debug,
    L2P: L2Provider + Clone + Debug,
{
    /// Drives the main batch submission loop.
    pub async fn loop(&self) -> Result<()> {
        for {
            // load cursor blocks
            let blocks = match self.cursor.load_blocks().await {
                Ok(b) => b,
                Err(e) => {
                    warn!(target: "batch-submitter", "Failed to load blocks: {e}");
                    continue;
                }
            };

            // TODO: 

        }
    }
}
