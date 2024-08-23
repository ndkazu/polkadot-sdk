
// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_opf`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Kazu-Rog`, CPU: `AMD Ryzen 9 4900HS with Radeon Graphics`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/substrate-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_opf
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// substrate/frame/opf/src/weights.rs
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --template
// substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_opf`.
pub trait WeightInfo {
	fn vote(r: u32, ) -> Weight;
	fn remove_vote(r: u32, ) -> Weight;
	fn unlock_funds(r: u32, ) -> Weight;
}

/// Weights for `pallet_opf` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `OptimisticProjectFunding::VotingRoundNumber` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::VotingRoundNumber` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::VotingRounds` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::VotingRounds` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::Votes` (r:2 w:1)
	/// Proof: `OptimisticProjectFunding::Votes` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::WhiteListedProjectAccounts` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::WhiteListedProjectAccounts` (`max_values`: Some(1), `max_size`: Some(2050), added: 2545, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 64]`.
	fn vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277 + r * (32 ±0)`
		//  Estimated: `6192`
		// Minimum execution time: 84_136_000 picoseconds.
		Weight::from_parts(85_531_315, 6192)
			// Standard Error: 7_241
			.saturating_add(Weight::from_parts(117_991, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `OptimisticProjectFunding::VotingRoundNumber` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::VotingRoundNumber` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::VotingRounds` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::VotingRounds` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::Votes` (r:1 w:1)
	/// Proof: `OptimisticProjectFunding::Votes` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 64]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492`
		//  Estimated: `3676`
		// Minimum execution time: 67_756_000 picoseconds.
		Weight::from_parts(69_024_099, 3676)
			// Standard Error: 1_038
			.saturating_add(Weight::from_parts(1_894, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `OptimisticProjectFunding::Votes` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::Votes` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 64]`.
	fn unlock_funds(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `3676`
		// Minimum execution time: 75_452_000 picoseconds.
		Weight::from_parts(97_430_164, 3676)
			// Standard Error: 6_339
			.saturating_add(Weight::from_parts(1_257, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `OptimisticProjectFunding::VotingRoundNumber` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::VotingRoundNumber` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::VotingRounds` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::VotingRounds` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::Votes` (r:2 w:1)
	/// Proof: `OptimisticProjectFunding::Votes` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::WhiteListedProjectAccounts` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::WhiteListedProjectAccounts` (`max_values`: Some(1), `max_size`: Some(2050), added: 2545, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 64]`.
	fn vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277 + r * (32 ±0)`
		//  Estimated: `6192`
		// Minimum execution time: 84_136_000 picoseconds.
		Weight::from_parts(85_531_315, 6192)
			// Standard Error: 7_241
			.saturating_add(Weight::from_parts(117_991, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `OptimisticProjectFunding::VotingRoundNumber` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::VotingRoundNumber` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::VotingRounds` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::VotingRounds` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `OptimisticProjectFunding::Votes` (r:1 w:1)
	/// Proof: `OptimisticProjectFunding::Votes` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 64]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492`
		//  Estimated: `3676`
		// Minimum execution time: 67_756_000 picoseconds.
		Weight::from_parts(69_024_099, 3676)
			// Standard Error: 1_038
			.saturating_add(Weight::from_parts(1_894, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `OptimisticProjectFunding::Votes` (r:1 w:0)
	/// Proof: `OptimisticProjectFunding::Votes` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 64]`.
	fn unlock_funds(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `3676`
		// Minimum execution time: 75_452_000 picoseconds.
		Weight::from_parts(97_430_164, 3676)
			// Standard Error: 6_339
			.saturating_add(Weight::from_parts(1_257, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
