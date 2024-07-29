//! Stages for batch submission.

mod l2_traversal;
pub use l2_traversal::L2Traversal;

mod l2_retrieval;
pub use l2_retrieval::{L2Retrieval, L2RetrievalProvider};

mod batch_processor;
pub use batch_processor::{BatchProcessor, BatchProcessorProvider};

mod channel_builder;
pub use channel_builder::{ChannelBuilder, ChannelBuilderProvider};

mod transaction_builder;
pub use transaction_builder::{TransactionBuilder, TransactionBuilderProvider};

// Stages:

// - L2Traversal: has a loop that can be run in a separate thread.
//   - Fetches the sync status from the rollup node
//     calculates the (start, end] range of L2 blocks
//     adds them to its state.
//   - If the sequencer sits in a closeby process,
//     the L2 blocks can be directly fed into the next
//     L2Retrieval stage.

// - L2Retrieval: accepts a list of L2 Blocks to submit
//   - next_l2_block()
//      - Calls on the L2Traversal stage for any new blocks
//      - Adds any of those blocks to stage
//      - Checks state for a new block
//      - Fetches the block by number on the L2 client
//      - responds with an L2 Block or `StageError::Eof`

//

// TransactionQueue: Returns the next transaction to batch submit
// -

//                           Batching Pipeline
//
//        add_blocks(...)                               next_tx()                         new_origin()
//               |                                          |                                  |
//               |                               TransactionBuilder.next_tx()       TransactionQueue.new_origin()
//               |                                          |                                  |
//               |                              ChannelBuilder.next_channel()    ChannelBuilder.new_origin() --> Checks that channel is not timed out
//               |                                          |
//               |                              BatchProcessor.next_batch()
//               |                                          |
//    L2Traversal.add_blocks(...)                L2Retrival.next_l2_block()
//               |                                          |
//                -----------------------------> L2Traversal.next_blocks()
