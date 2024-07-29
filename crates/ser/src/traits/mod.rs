//! Contains trait definitions.

mod rollup;
pub use rollup::RollupNode;

mod providers;
pub use providers::{L1Provider, L2Provider};

mod compressor;
pub use compressor::Compressor;

mod stages;
pub use stages::{FramePublished, NextTransaction, OriginReceiver};
