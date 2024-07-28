//! Channel Builder

use alloc::boxed::Box;
use async_trait::async_trait;
use kona_derive::types::SingleBatch;

use crate::stages::FrameQueueProvider;
use crate::types::ChannelOut;

/// The provider for the channel builder stage.
#[async_trait]
pub trait ChannelBuilderProvider {
    /// Returns the next [SingleBatch].
    async fn next_batch(&mut self) -> Option<SingleBatch>;
}

/// The [ChannelBuilder] stage of the batching pipeline.
#[derive(Debug, Clone)]
pub struct ChannelBuilder<P: ChannelBuilderProvider> {
    /// The previous stage.
    pub prev: P,
    /// Whether to build span or single channels.
    pub span: bool,
}

#[async_trait]
impl<P: ChannelBuilderProvider + Send> FrameQueueProvider for ChannelBuilder<P> {
    async fn next_channel(&mut self) -> Option<ChannelOut> {
        unimplemented!()
    }
}
