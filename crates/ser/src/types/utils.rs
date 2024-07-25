//! Contains utilities for working with types and conversions.

use anyhow::Result;
use alloy_rpc_types::{Block, Header};
use kona_derive::types::SingleBatch;

/// Converts a [Block] into a [SingleBatch].
pub fn block_to_batch(cfg: &RollupConfig, block: &Block) -> Result<SingleBatch> {
    
}


// BlockToSingularBatch transforms a block into a batch object that can easily be RLP encoded.
func BlockToSingularBatch(rollupCfg *rollup.Config, block *types.Block) (*SingularBatch, *L1BlockInfo, error) {
	if len(block.Transactions()) == 0 {
		return nil, nil, fmt.Errorf("block %v has no transactions", block.Hash())
	}

	opaqueTxs := make([]hexutil.Bytes, 0, len(block.Transactions()))
	for i, tx := range block.Transactions() {
		if tx.Type() == types.DepositTxType {
			continue
		}
		otx, err := tx.MarshalBinary()
		if err != nil {
			return nil, nil, fmt.Errorf("could not encode tx %v in block %v: %w", i, tx.Hash(), err)
		}
		opaqueTxs = append(opaqueTxs, otx)
	}

	l1InfoTx := block.Transactions()[0]
	if l1InfoTx.Type() != types.DepositTxType {
		return nil, nil, ErrNotDepositTx
	}
	l1Info, err := L1BlockInfoFromBytes(rollupCfg, block.Time(), l1InfoTx.Data())
	if err != nil {
		return nil, l1Info, fmt.Errorf("could not parse the L1 Info deposit: %w", err)
	}

	return &SingularBatch{
		ParentHash:   block.ParentHash(),
		EpochNum:     rollup.Epoch(l1Info.Number),
		EpochHash:    l1Info.BlockHash,
		Timestamp:    block.Time(),
		Transactions: opaqueTxs,
	}, l1Info, nil
}
