//! Frame Queue

use alloc::boxed::Box;
use async_trait::async_trait;
use kona_derive::types::Frame;

use crate::stages::TransactionBuilderProvider;
use crate::types::ChannelOut;

/// The provider for the frame queue stage.
#[async_trait]
pub trait FrameQueueProvider {
    /// Returns the next [ChannelOut].
    async fn next_channel(&mut self) -> Option<ChannelOut>;
}

/// The [FrameQueue] stage of the batching pipeline.
#[derive(Debug, Clone)]
pub struct FrameQueue<P: FrameQueueProvider> {
    /// The previous stage.
    pub prev: P,
}

#[async_trait]
impl<P: FrameQueueProvider + Send> TransactionBuilderProvider for FrameQueue<P> {
    async fn next_frame(&mut self) -> Option<Frame> {
        unimplemented!()
    }
}
