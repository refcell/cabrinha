//! Sync types for the optimism rollup node.

/// The [`SyncStatus`][ss] of an Optimism Rollup Node.
///
/// [ss]: https://github.com/ethereum-optimism/optimism/blob/develop/op-service/eth/sync_status.go#L5
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncStatus {
    /// The current L1 block.
    pub current_l1: L1BlockRef,
    /// The current L1 finalized block.
    pub current_l1_finalized: L1BlockRef,
    /// The L1 head block ref.
    pub head_l1: L1BlockRef,
    /// The L1 safe head block ref.
    pub safe_l1: L1BlockRef,
    /// The finalized L1 block ref.
    pub finalized_l1: L1BlockRef,
    /// The unsafe L2 block ref.
    pub unsafe_l2: L2BlockRef,
    /// The safe L2 block ref.
    pub safe_l2: L2BlockRef,
    /// The finalized L2 block ref.
    pub finalized_l2: L2BlockRef,
    /// The pending safe L2 block ref.
    pub pending_safe_l2: L2BlockRef,
}
