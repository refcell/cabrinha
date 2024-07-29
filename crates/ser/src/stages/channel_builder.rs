//! Channel Builder

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use async_trait::async_trait;
use kona_derive::types::Frame;
use kona_derive::types::RollupConfig;
use kona_derive::types::SingleBatch;

use crate::stages::TransactionBuilderProvider;
use crate::traits::{Compressor, FramePublished, OriginReceiver};
use crate::types::{ChannelOut, L1BlockRef};

/// The provider for the channel builder stage.
#[async_trait]
pub trait ChannelBuilderProvider {
    /// Returns the next [SingleBatch].
    async fn next_batch(&mut self) -> Option<SingleBatch>;
}

/// The [ChannelBuilder] stage of the batching pipeline.
#[derive(Debug, Clone)]
pub struct ChannelBuilder<P: ChannelBuilderProvider, C: Compressor + Clone> {
    /// The previous stage.
    pub prev: P,
    /// Whether to build span or single channels.
    pub span: bool,
    /// The rollup config to use.
    pub rollup_config: Arc<RollupConfig>,
    /// Current [ChannelOut] state.
    pub channels: Vec<ChannelOut<C>>,
    /// The compressor to use.
    pub compressor: C,
    /// The configured sub safety margin.
    pub sub_safety_margin: u64,
}

impl<P: ChannelBuilderProvider, C: Compressor + Clone> ChannelBuilder<P, C> {
    /// Constructs a new [ChannelBuilder].
    pub fn new(
        prev: P,
        span: bool,
        cfg: Arc<RollupConfig>,
        compressor: C,
        sub_safety_margin: u64,
    ) -> Self {
        Self {
            prev,
            span,
            rollup_config: cfg,
            channels: vec![],
            compressor,
            sub_safety_margin,
        }
    }
}

impl<P: ChannelBuilderProvider + Send, C: Compressor + Send + Clone> OriginReceiver
    for ChannelBuilder<P, C>
{
    fn set_origin(&mut self, origin: L1BlockRef) {
        for c in self.channels.iter_mut() {
            c.check_timed_out(origin.number)
        }
    }
}

impl<P: ChannelBuilderProvider + Send, C: Compressor + Send + Clone> FramePublished
    for ChannelBuilder<P, C>
{
    fn frame_published(&mut self, l1_block_num: u64) {
        for c in self.channels.iter_mut() {
            c.frame_published(l1_block_num)
        }
    }
}

#[async_trait]
impl<P: ChannelBuilderProvider + Send, C: Compressor + Send + Clone> TransactionBuilderProvider
    for ChannelBuilder<P, C>
{
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
            let mut c = ChannelOut::new(
                Arc::clone(&self.rollup_config),
                self.compressor.clone(),
                batch.epoch_num,
                self.sub_safety_margin,
            );
            c.add_batch(batch);
            self.channels.push(c);
        }

        None
    }
}
