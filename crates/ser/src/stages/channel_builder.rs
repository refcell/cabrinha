//! Channel Builder

use alloc::boxed::Box;
use async_trait::async_trait;
use kona_derive::types::SingleBatch;

/// The provider for the channel builder stage.
#[async_trait]
pub trait ChannelBuilderProvider {
    /// Returns the next [SingleBatch].
    async fn next_batch(&mut self) -> Option<SingleBatch>;
}
