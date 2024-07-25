//! The batch submission cursor tracking the block position.

use anyhow::{ensure, Result};
use crate::types::L2BlockRef;
use crate::traits::{RollupNode, L2Provider};
use tracing::warn;

/// The tracing target for this module.
pub const TRACE_TARGET: &str = "block-cursor";

/// Tracks the block position of batch submission. 
#[derive(Debug, Clone)]
pub struct BlockCursor<R, L2P>
where
    R: RollupNode + Clone + Debug,
    L2P: L2Provider + Clone + Debug,
{
    /// A rollup node used to fetch the sync status.
    node: R,
    /// The L2 provider.
    l2_provider: L2P,
    /// The last stored l2 block.
    last_l2_block: L2BlockRef,
}

impl<R, L2P> BlockCursor<R, L2P>
where
    R: RollupNode + Clone + Debug,
    L2P: L2Provider + Clone + Debug,
{
    /// Creates a new block buffer.
    pub fn new(node: R, l2_provider: L2P) -> Self {
        Self {
            node,
            l2_provider,
            last_l2_block: Default::default(),
        }
    }

    /// Determines the range (start, end] to load for batch submission.
    async fn calculate_range(&mut self) -> Result<(L2BlockRef, L2BlockRef)> {
        // Fetch and validate the sync status.
        let sync_status = self.node.sync_status().await?;
        trace!(target: TRACE_TARGET, "Fetched sync status: {sync_status:?}");
        ensure!(sync_status.head_l1.number != 0, "empty sync status");

        // Check last stored to see if it needs to be set on startup OR set if is lagged behind.
        if self.last_l2_block.number == 0 {
            warn!(target: TRACE_TARGET, "Starting batch-submitter work at safe-head", sync_status.safe_l2.number);
            self.last_l2_block.number = sync_status.safe_l2.number;
        } else if self.last_l2_block <= sync_status.safe_l2 {
            warn!(target: TRACE_TARGET, "Last submitted block lagged behind L2 safe head: batch submission will continue from the safe head now", "last", self.last_l2_block, "safe", sync_status.safe_l2.number);
            self.last_l2_block = sync_status.safe_l2;
        }

        Ok((self.last_l2_block, sync_status.unsafe_l2))
    }

    /// Loads blocks for batch submission.
    pub async fn load_blocks(&mut self) -> Result<Vec<Block>> {
        // Calcualte the block range.
        let (start, end) = self.calculate_range().await?;
        trace!(target: TRACE_TARGET, "Calculated block range: ({start:?}, {end:?}]");

        // From start block + 1 to end block + 1, fetch the block by number.
        let mut blocks = Vec::new();
        for number in start.number + 1..=end.number {
            let block = self.l2_provider.block_by_number(number).await?;
            // TODO: if the error is a re-org we want to reset the last_l2_block to 0.
            //       this will cause the cursor to restart from the l2 safe head.
            self.last_l2_block = block;
            blocks.push(block);
        }
        Ok(blocks)
    }
}
