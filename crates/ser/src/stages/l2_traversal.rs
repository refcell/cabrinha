//! L2 Traversal Stage

use crate::stages::L2RetrievalProvider;
use crate::traits::{L2Provider, RollupNode};
use crate::types::L2BlockRef;
use alloc::boxed::Box;
use alloc::vec::Vec;
use anyhow::Result;
use async_trait::async_trait;
use tracing::warn;

/// The [L2Traversal] stage of the batching pipeline.
///
/// This stage sits at the bottom or input of the pipeline.
#[derive(Debug, Clone)]
pub struct L2Traversal<R: RollupNode, P: L2Provider> {
    /// The l2 block cursor
    pub cursor: L2BlockRef,
    /// A list of L2 Blocks in the traversal stage
    pub blocks: Vec<L2BlockRef>,
    /// The rollup node to fetch sync status from.
    pub rollup_node: R,
    /// The l2 provider to fetch [L2BlockRef] from.
    pub l2_provider: P,
}

#[async_trait]
impl<R: RollupNode + Send, P: L2Provider + Send> L2RetrievalProvider for L2Traversal<R, P> {
    async fn next_blocks(&mut self) -> Vec<L2BlockRef> {
        self.blocks.drain(0..).collect()
    }
}

impl<R: RollupNode, P: L2Provider> L2Traversal<R, P> {
    /// Attempts to load blocks using the rollup node.
    pub async fn load_blocks(&mut self) -> Result<()> {
        // Fetch the sync status from the rollup node.
        let ss = self.rollup_node.sync_status().await?;

        // If the sync status unsafe L2 head is prior to the cursor, ignore.
        if ss.unsafe_l2.number < self.cursor.number {
            warn!(target: "l2-traversal", "Unsafe L2 Head {:?} Behind Cursor {:?}", ss.unsafe_l2, self.cursor);
            return Ok(());
        }

        // If the cursor is prior to the safe l2 head,
        // batching just started or is lagging other producers.
        // In any case, update blocks with the new sync status.
        let mut new_cursor = self.cursor.clone();
        if new_cursor.number < ss.safe_l2.number {
            // Remove any blocks that are before the current
            // l2 safe head since they've already been batch submitted.
            let before_safe_head = self
                .blocks
                .iter()
                .position(|b| b.number >= ss.safe_l2.number);
            if let Some(index) = before_safe_head {
                self.blocks.drain(0..index);
            }
            // TODO: Metrice if the l2 traversal stage lags behind the safe l2 head.
            warn!(target: "l2-traversal", "Cursor {:?} Behind L2 Safe Head {:?}", self.cursor, ss.safe_l2);
            new_cursor = ss.safe_l2;
        }

        // If the cursor is equal to or before the unsafe l2 head, update local
        // block list with the blocks.
        let start_block = new_cursor.number + 1;
        let block_len = if ss.unsafe_l2.number < start_block {
            0
        } else {
            ss.unsafe_l2.number - new_cursor.number
        };
        let mut blocks = Vec::with_capacity(block_len as usize);
        for i in start_block..ss.unsafe_l2.number {
            // Check if the block is already tracked in the local state.
            if self.blocks.iter().any(|b| b.number == i) {
                continue;
            }
            blocks.push(self.l2_provider.block_ref_by_number(i).await?);
        }
        self.blocks.append(&mut blocks);
        self.cursor = ss.unsafe_l2;
        Ok(())
    }

    /// Adds blocks to the [L2Traversal] stage, validating the new range with the cursor.
    pub fn add_blocks(&mut self, new_blocks: &[L2BlockRef]) {
        if new_blocks.is_empty() {
            return;
        }
        let mut blocks = Vec::with_capacity(new_blocks.len());
        for block in new_blocks {
            if self.blocks.iter().any(|b| b.number == block.number) {
                continue;
            }
            blocks.push(block.clone());
        }
        if blocks.is_empty() {
            return;
        }
        if let Some(b) = blocks.last().cloned() {
            self.cursor = b.clone();
        }
        self.blocks.append(&mut blocks);
    }
}
