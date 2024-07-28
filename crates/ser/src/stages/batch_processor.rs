//! Batch Processor

use alloc::boxed::Box;
use alloy_rpc_types::Block;
use async_trait::async_trait;

/// The provider for the batch processor stage.
#[async_trait]
pub trait BatchProcessorProvider {
    /// Returns the next L2 [Block].
    async fn next_l2_block(&mut self) -> Option<Block>;
}

/// The [BatchProcessor] stage of the batching pipeline.
#[derive(Debug, Clone)]
pub struct BatchProcessor {}
