//! Channel Builder

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use async_trait::async_trait;
use kona_derive::types::Frame;
use kona_derive::types::SingleBatch;

use crate::stages::TransactionBuilderProvider;
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
    /// Current [ChannelOut] state.
    pub channels: Vec<ChannelOut>,
}

impl<P: ChannelBuilderProvider> ChannelBuilder<P> {
    /// Constructs a new [ChannelBuilder].
    pub fn new(prev: P, span: bool) -> Self {
        Self {
            prev,
            span,
            channels: vec![],
        }
    }
}

#[async_trait]
impl<P: ChannelBuilderProvider + Send> TransactionBuilderProvider for ChannelBuilder<P> {
    async fn next_frame(&mut self) -> Option<Frame> {
        // If all frames in the [ChannelOut] are output,
        // remove the channel from the state.
        if self.channels.first().is_none() {
            self.channels.remove(0);
        }

        // If the frame is ready, output the next frame.
        if let Some(c) = self.channels.first_mut() {
            if c.frame_ready() {
                return c.output_frame();
            }
        }

        // If the frame is not ready, try to add a new batch to the channel.
        if let Some(batch) = self.prev.next_batch().await {
            let mut c = ChannelOut::new();
            c.add_batch(batch);
            self.channels.push(c);
        }

        None
    }
}
