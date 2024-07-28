//! Transaction Builder

use alloc::boxed::Box;
use alloy_rpc_types::TransactionRequest;
use async_trait::async_trait;
use kona_derive::types::Frame;

/// Trait for producing the next transactions.
#[async_trait]
pub trait NextTransaction {
    /// Returns the next [TransactionRequest].
    async fn next_transaction(&mut self) -> Option<TransactionRequest>;
}

/// The provider for the transaction builder stage.
#[async_trait]
pub trait TransactionBuilderProvider {
    /// Returns the next [Frame].
    async fn next_frame(&mut self) -> Option<Frame>;
}

/// The [TransactionBuilder] stage of the batching pipeline.
#[derive(Debug, Clone)]
pub struct TransactionBuilder<P: TransactionBuilderProvider> {
    /// The previous stage.
    pub prev: P,
}

#[async_trait]
impl<P: TransactionBuilderProvider + Send> NextTransaction for TransactionBuilder<P> {
    async fn next_transaction(&mut self) -> Option<TransactionRequest> {
        unimplemented!()
    }
}
