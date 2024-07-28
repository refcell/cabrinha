#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

extern crate alloc;

mod traits;
pub use traits::{L1Provider, L2Provider, RollupNode};

mod types;
pub use types::{
    block_to_batch, l1_block_info_from_bytes, BatchTransaction, ChannelOut, L1BlockRef, L2BlockRef,
    SyncStatus,
};

mod stages;
pub use stages::{
    BatchProcessor, BatchProcessorProvider, ChannelBuilder, ChannelBuilderProvider, L2Retrieval,
    L2RetrievalProvider, L2Traversal, NextTransaction, TransactionBuilder,
    TransactionBuilderProvider,
};
