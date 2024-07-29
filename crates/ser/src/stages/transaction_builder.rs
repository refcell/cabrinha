//! Transaction Builder

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use async_trait::async_trait;
use kona_derive::types::Frame;

use crate::traits::{FramePublished, NextTransaction, OriginReceiver};
use crate::types::{BatchTransaction, L1BlockRef};

/// The provider for the transaction builder stage.
#[async_trait]
pub trait TransactionBuilderProvider {
    /// Returns the next [Frame].
    async fn next_frame(&mut self) -> Option<Frame>;
}

/// The [TransactionBuilder] stage of the batching pipeline.
#[derive(Debug, Clone)]
pub struct TransactionBuilder<P: TransactionBuilderProvider + OriginReceiver + FramePublished> {
    /// The previous stage.
    pub prev: P,
    /// Max number of frames to allow per transaction.
    pub max_frames: u16,
    /// Batch transactions.
    pub txs: Vec<BatchTransaction>,
}

impl<P: TransactionBuilderProvider + OriginReceiver + FramePublished> TransactionBuilder<P> {
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

impl<P: TransactionBuilderProvider + OriginReceiver + FramePublished + Send> OriginReceiver
    for TransactionBuilder<P>
{
    fn set_origin(&mut self, origin: L1BlockRef) {
        self.prev.set_origin(origin);
    }
}

impl<P: TransactionBuilderProvider + OriginReceiver + FramePublished + Send> FramePublished
    for TransactionBuilder<P>
{
    fn frame_published(&mut self, l1_block_num: u64) {
        self.prev.frame_published(l1_block_num)
    }
}

#[async_trait]
impl<P: TransactionBuilderProvider + Send + OriginReceiver + FramePublished> NextTransaction
    for TransactionBuilder<P>
{
    async fn next_transaction(&mut self) -> Option<&BatchTransaction> {
        while let Some(f) = self.prev.next_frame().await {
            self.add_frame(f);
        }

        return self.txs.first();
    }
}
