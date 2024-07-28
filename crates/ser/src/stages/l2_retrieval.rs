//! L2 Retrieval Stage

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloy_rpc_types::Block;
use async_trait::async_trait;
use tracing::warn;

use crate::stages::BatchProcessorProvider;
use crate::traits::L2Provider;
use crate::types::L2BlockRef;

/// The provider for the retrieval stage.
#[async_trait]
pub trait L2RetrievalProvider {
    /// Returns a list of [L2BlockRef].
    async fn next_blocks(&mut self) -> Vec<L2BlockRef>;
}

/// The [L2Retrieval] stage of the batching pipeline.
#[derive(Debug, Clone)]
pub struct L2Retrieval<R: L2RetrievalProvider, P: L2Provider> {
    /// Handle to the previous stage.
    pub prev: R,
    /// The L2 Provider.
    pub provider: P,
    /// Contains L2 Blocks.
    pub blocks: Vec<Block>,
}

#[async_trait]
impl<R: L2RetrievalProvider + Send, P: L2Provider + Send> BatchProcessorProvider
    for L2Retrieval<R, P>
{
    async fn next_l2_block(&mut self) -> Option<Block> {
        let block_refs = self.prev.next_blocks().await;
        for block_ref in block_refs {
            match self.provider.block_by_number(block_ref.number).await {
                Ok(block) => self.blocks.push(block),
                Err(e) => {
                    warn!(target: "l2-retrieval", "Failed to fetch block: {:?}", e);
                    break;
                }
            }
        }
        if self.blocks.is_empty() {
            return None;
        }
        Some(self.blocks.remove(0))
    }
}
