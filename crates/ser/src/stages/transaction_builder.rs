//! Transaction Builder

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use async_trait::async_trait;
use kona_derive::types::Frame;

use crate::types::BatchTransaction;

/// Trait for producing the next transactions.
#[async_trait]
pub trait NextTransaction {
    /// Returns the next [BatchTransaction].
    async fn next_transaction(&mut self) -> Option<&BatchTransaction>;
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
    /// Max number of frames to allow per transaction.
    pub max_frames: u16,
    /// Batch transactions.
    pub txs: Vec<BatchTransaction>,
}

impl<P: TransactionBuilderProvider> TransactionBuilder<P> {
    /// Constructs a new [TransactionBuilder].
    pub fn new(prev: P, max_frames: u16) -> Self {
        Self {
            prev,
            max_frames,
            txs: vec![],
        }
    }

    /// Adds a frame to the transactions queue.
    /// Finds the first transaction that is not full and adds the frame to it.
    /// If no transaction is found, a new transaction is created.
    fn add_frame(&mut self, frame: Frame) {
        let tx = self.txs.iter_mut().find(|tx| !tx.is_full(self.max_frames));
        match tx {
            Some(tx) => {
                tx.size += frame.size();
                tx.frames.push(frame);
            }
            None => {
                self.txs.push(BatchTransaction {
                    size: frame.size(),
                    frames: vec![frame],
                });
            }
        }
    }
}

#[async_trait]
impl<P: TransactionBuilderProvider + Send> NextTransaction for TransactionBuilder<P> {
    async fn next_transaction(&mut self) -> Option<&BatchTransaction> {
        while let Some(f) = self.prev.next_frame().await {
            self.add_frame(f);
        }

        return self.txs.first();
    }
}
