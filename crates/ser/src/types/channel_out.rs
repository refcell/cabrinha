//! Contains logic for constructing a channel.

use alloc::vec::Vec;
use alloy_rlp::Encodable;
use kona_derive::types::Frame;
use kona_derive::types::RollupConfig;
use kona_derive::types::SingleBatch;
use kona_derive::ChannelID;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use tracing::warn;

/// The absolute minimum size of a frame.
/// This is the fixed overhead frame size, calculated as specified
/// in the [Frame Format][ff] specs: 16 + 2 + 4 + 1 = 23 bytes.
///
/// [ff]: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#frame-format
pub(crate) const FRAME_V0_OVERHEAD_SIZE: u64 = 23;

/// The channel output type.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChannelOut {
    /// The current channel id.
    pub id: ChannelID,
    /// The current frame number.
    pub frame: u16,
    /// The uncompressed size of the channel.
    /// This must be less than [kona_derive::MAX_RLP_BYTES_PER_CHANNEL].
    pub rlp_length: u64,
    /// If the channel is closed.
    pub closed: bool,
    /// The configured max channel size.
    /// This should come from the Chain Spec.
    pub max_frame_size: u64,
    /// The chain config.
    pub chain_config: RollupConfig,
}

impl ChannelOut {
    /// Constructs a new [ChannelOut].
    pub fn new() -> Self {
        let mut small_rng = SmallRng::from_entropy();
        let mut id = ChannelID::default();
        small_rng.fill(&mut id);
        Self {
            id,
            ..Self::default()
        }
    }

    /// Returns the ready bytes from the channel.
    pub fn ready_bytes(&self) -> u64 {
        // TODO: get the len of the compressor
        unimplemented!()
    }

    /// Max RLP Bytes per channel.
    /// This is retrieved from the Chain Spec since it changes after the Fjord Hardfork.
    /// Uses the batch timestamp to determine the max RLP bytes per channel.
    pub fn max_rlp_bytes_per_channel(&self, _batch: &SingleBatch) -> u64 {
        // self.chain_config.max_rlp_bytes_per_channel(batch.timestamp);
        unimplemented!()
    }

    /// Adds a batch to the [ChannelOut].
    pub fn add_batch(&mut self, batch: SingleBatch) {
        if self.closed {
            warn!(target: "channel-out", "Channel is closed. Not adding batch: {:?}", self.id);
            return;
        }

        // RLP encode the batch
        let mut buf = Vec::new();
        batch.encode(&mut buf);

        let max_per_channel = self.max_rlp_bytes_per_channel(&batch);
        if self.rlp_length + buf.len() as u64 > max_per_channel {
            warn!(target: "channel-out", "Batch exceeds max RLP bytes per channel ({}). Closing channel: {:?}", max_per_channel, self.id);
            self.closed = true;
            return;
        }

        self.rlp_length += buf.len() as u64;

        // TODO: write the buffer to the compressor
    }

    /// Checks if a frame has enough bytes to output.
    pub fn frame_ready(&self) -> bool {
        self.ready_bytes() + FRAME_V0_OVERHEAD_SIZE >= self.max_frame_size
    }

    /// Compress the channel to produce frame bytes.
    pub fn compress(&mut self) -> Vec<u8> {
        unimplemented!()
    }

    /// Outputs the next frame if available.
    pub fn output_frame(&mut self) -> Option<Frame> {
        if !self.frame_ready() {
            return None;
        }

        let frame = Frame {
            id: self.id,
            number: self.frame,
            data: self.compress(),
            is_last: false,
        };
        self.frame += 1;

        // If the max frame number is reached,
        // the channel must be closed.
        if self.frame == u16::MAX {
            warn!(target: "channel-out", "Max frame number reached ({}). Closed channel: ({:?})", self.frame, self.id);
            self.closed = true;
        }

        Some(frame)
    }
}
