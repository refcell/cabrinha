//! Contains utilities for working with types and conversions.

use alloc::vec::Vec;
use alloy_consensus::TxEnvelope;
use alloy_eips::eip2718::Encodable2718;
use alloy_rpc_types::{Block, BlockTransactions};
use anyhow::{ensure, Result};
use kona_derive::types::{L1BlockInfoTx, RawTransaction, RollupConfig, SingleBatch};

/// Converts a [Block] into a [SingleBatch].
pub fn block_to_batch(cfg: &RollupConfig, block: &Block) -> Result<(SingleBatch, L1BlockInfoTx)> {
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
    let l1_info = l1_block_info_from_bytes(cfg, block.header.timestamp, &buf)?;

    Ok((
        SingleBatch {
            parent_hash: block.header.parent_hash,
            epoch_num: l1_info.id().number,
            epoch_hash: l1_info.id().hash,
            timestamp: block.header.timestamp,
            transactions: raw_txs,
        },
        l1_info,
    ))
}

/// Constructs an [L1BlockInfoTx] from the raw transaction bytes.
pub fn l1_block_info_from_bytes(
    _rollup_cfg: &RollupConfig,
    _block_time: u64,
    _data: &[u8],
) -> Result<L1BlockInfoTx> {
    // go: L1BlockInfoFromBytes
    unimplemented!()
}
