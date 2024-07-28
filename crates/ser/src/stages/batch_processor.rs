//! Batch Processor

use alloc::boxed::Box;
use alloy_rpc_types::Block;
use async_trait::async_trait;
use kona_derive::types::RollupConfig;
use kona_derive::types::SingleBatch;
use tracing::error;

use crate::block_to_batch;
use crate::stages::ChannelBuilderProvider;

/// The provider for the batch processor stage.
#[async_trait]
pub trait BatchProcessorProvider {
    /// Returns the next L2 [Block].
    async fn next_l2_block(&mut self) -> Option<Block>;
}

/// The [BatchProcessor] stage of the batching pipeline.
#[derive(Debug, Clone)]
pub struct BatchProcessor<P: BatchProcessorProvider> {
    /// The previous stage.
    pub prev: P,
    /// The rollup config to use for constructing a batch from a block.
    pub rollup_config: RollupConfig,
}

#[async_trait]
impl<P: BatchProcessorProvider + Send> ChannelBuilderProvider for BatchProcessor<P> {
    async fn next_batch(&mut self) -> Option<SingleBatch> {
        let block = self.prev.next_l2_block().await?;
        match block_to_batch(&self.rollup_config, &block) {
            Ok((batch, _l1_info)) => Some(batch),
            Err(e) => {
                error!(target: "batch-processor", "Failed to convert block to batch: {:?}", e);
                None
            }
        }
    }
}
