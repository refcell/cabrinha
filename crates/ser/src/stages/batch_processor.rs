//! Batch Processor

use alloc::boxed::Box;
use alloc::vec::Vec;
use anyhow::{ensure, Result};
use async_trait::async_trait;
use tracing::error;

use alloy_consensus::TxEnvelope;
use alloy_eips::eip2718::Encodable2718;
use alloy_rpc_types::{Block, BlockTransactions};
use kona_derive::types::{L1BlockInfoTx, RawTransaction, SingleBatch};

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
}

impl<P: BatchProcessorProvider> BatchProcessor<P> {
    /// Constructs a new [BatchProcessor].
    pub fn new(prev: P) -> Self {
        Self { prev }
    }

    /// Converts a [Block] to a [SingleBatch].
    fn block_to_batch(&self, block: &Block) -> Result<SingleBatch> {
        // Ensure the block has transactions
        ensure!(
            !block.transactions.is_empty(),
            "block {:?} has no transactions",
            block.header.hash
        );
        let txs = match block.transactions {
            BlockTransactions::Full(ref txs) => txs,
            _ => {
                return Err(anyhow::anyhow!(
                    "missing full transactions for block: {:?}",
                    block.header.hash
                ))
            }
        };

        // Encode the transactions to raw bytes
        let mut raw_txs = Vec::with_capacity(txs.len());
        for tx in txs {
            // Skip deposit transactions
            if tx.transaction_type == Some(0x7E) {
                continue;
            }
            let envelope = TxEnvelope::try_from(tx.clone()).map_err(|e| anyhow::anyhow!(e))?;
            let mut buf = Vec::new();
            envelope.encode_2718(&mut buf);
            raw_txs.push(RawTransaction(buf.into()));
        }

        // Parse the L1 Info transaction
        let l1_info_tx = match txs.first() {
            Some(tx) => tx,
            None => {
                return Err(anyhow::anyhow!(
                    "block {:?} has no transactions",
                    block.header.hash
                ))
            }
        };
        ensure!(
            l1_info_tx.transaction_type == Some(0x7E),
            "first transaction in block (l1 info tx) {:?} is not a deposit",
            block.header.hash
        );
        let envelope = TxEnvelope::try_from(l1_info_tx.clone()).map_err(|e| anyhow::anyhow!(e))?;
        let mut buf = Vec::new();
        envelope.encode_2718(&mut buf);
        let l1_info = L1BlockInfoTx::decode_calldata(&buf)?;

        Ok(SingleBatch {
            parent_hash: block.header.parent_hash,
            epoch_num: l1_info.id().number,
            epoch_hash: l1_info.id().hash,
            timestamp: block.header.timestamp,
            transactions: raw_txs,
        })
    }
}

#[async_trait]
impl<P: BatchProcessorProvider + Send> ChannelBuilderProvider for BatchProcessor<P> {
    async fn next_batch(&mut self) -> Option<SingleBatch> {
        let block = self.prev.next_l2_block().await?;
        match self.block_to_batch(&block) {
            Ok(batch) => Some(batch),
            Err(e) => {
                error!(target: "batch-processor", "Failed to convert block to batch: {:?}", e);
                None
            }
        }
    }
}
