// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod metadata;
pub use metadata::*;

mod bytes;
mod genesis;
mod merkle;
mod serialize;
mod string;

use crate::block::Transactions;
use console::{
    network::prelude::*,
    program::{HeaderLeaf, HeaderPath, HeaderTree, HEADER_DEPTH},
    types::Field,
};

/// The header for the block contains metadata that uniquely identifies the block.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Header<N: Network> {
    /// The Merkle root representing the blocks in the ledger up to the previous block.
    previous_state_root: Field<N>,
    /// The Merkle root representing the transactions in the block.
    transactions_root: Field<N>,
    /// The Merkle root representing the on-chain finalize including the current block.
    finalize_root: Field<N>,
    /// The accumulator point of the coinbase puzzle.
    coinbase_accumulator_point: Field<N>,
    /// The metadata of the block.
    metadata: Metadata<N>,
}

impl<N: Network> Header<N> {
    /// Initializes a new block header with the given inputs.
    pub fn from(
        previous_state_root: Field<N>,
        transactions_root: Field<N>,
        finalize_root: Field<N>,
        coinbase_accumulator_point: Field<N>,
        metadata: Metadata<N>,
    ) -> Result<Self> {
        // Construct a new block header.
        let header =
            Self { previous_state_root, transactions_root, finalize_root, coinbase_accumulator_point, metadata };
        // Ensure the header is valid.
        match header.is_valid() {
            true => Ok(header),
            false => bail!("Invalid block header: {:?}", header),
        }
    }

    /// Returns `true` if the block header is well-formed.
    pub fn is_valid(&self) -> bool {
        match self.height() == 0u32 {
            true => self.is_genesis(),
            false => {
                // Ensure the previous ledger root is nonzero.
                self.previous_state_root != Field::zero()
                    // Ensure the transactions root is nonzero.
                    && self.transactions_root != Field::zero()
                    // Ensure the metadata is valid.
                    && self.metadata.is_valid()
            }
        }
    }

    /// Returns the previous state root from the block header.
    pub const fn previous_state_root(&self) -> Field<N> {
        self.previous_state_root
    }

    /// Returns the transactions root in the block header.
    pub const fn transactions_root(&self) -> Field<N> {
        self.transactions_root
    }

    /// Returns the finalize root in the block header.
    pub const fn finalize_root(&self) -> Field<N> {
        self.finalize_root
    }

    /// Returns the coinbase puzzle accumulator point in the block header.
    pub const fn coinbase_accumulator_point(&self) -> Field<N> {
        self.coinbase_accumulator_point
    }

    /// Returns the metadata in the block header.
    pub const fn metadata(&self) -> &Metadata<N> {
        &self.metadata
    }

    /// Returns the network ID of the block.
    pub const fn network(&self) -> u16 {
        self.metadata.network()
    }

    /// Returns the round number of the block.
    pub const fn round(&self) -> u64 {
        self.metadata.round()
    }

    /// Returns the height of the block.
    pub const fn height(&self) -> u32 {
        self.metadata.height()
    }

    /// Returns the total supply of microcredits at this block.
    pub const fn total_supply_in_microcredits(&self) -> u64 {
        self.metadata.total_supply_in_microcredits()
    }

    /// Returns the cumulative weight for this block.
    pub const fn cumulative_weight(&self) -> u128 {
        self.metadata.cumulative_weight()
    }

    /// Returns the coinbase target for this block.
    pub const fn coinbase_target(&self) -> u64 {
        self.metadata.coinbase_target()
    }

    /// Returns the proof target for this block.
    pub const fn proof_target(&self) -> u64 {
        self.metadata.proof_target()
    }

    /// Returns the coinbase target of the last coinbase.
    pub const fn last_coinbase_target(&self) -> u64 {
        self.metadata.last_coinbase_target()
    }

    /// Returns the Unix timestamp (UTC) of the last coinbase.
    pub const fn last_coinbase_timestamp(&self) -> i64 {
        self.metadata.last_coinbase_timestamp()
    }

    /// Returns the Unix timestamp (UTC) for this block.
    pub const fn timestamp(&self) -> i64 {
        self.metadata.timestamp()
    }
}
