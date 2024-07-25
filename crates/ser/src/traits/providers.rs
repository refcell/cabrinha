//! Providers for the chain.

use anyhow::Result;
use async_trait::async_trait;
use alloy_primitives::Address;
use alloy_rpc_types::{Block, Header};

/// A provider for the L1 Chain.
#[async_trait]
pub trait L1Provider {
    /// Fetches the header of the block with the given number.
    async fn header_by_number(&self, number: u64) -> Result<Header>;

    /// Fetches the nonce of the account at the given block number.
    async fn nonce_at(&self, account: Address, block_number: u64) -> Result<u64>;
}

/// A provider for the L2 Chain.
#[async_trait]
pub trait L2Provider {
    /// Fetches the block with the given number.
    async fn block_by_number(&self, number: u64) -> Result<Block>;
}
