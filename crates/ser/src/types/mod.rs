//! Contains types for the cabrinha serialization.

mod sync;
pub use sync::{L1BlockRef, L2BlockRef, SyncStatus};

mod utils;
pub use utils::block_to_batch;
