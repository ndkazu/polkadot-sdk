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
//
//! Autogenerated weights for `pallet_distribution`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-10-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_distribution
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// substrate/frame/distribution/src/weights.rs
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

/// Weight functions needed for `pallet_distribution`.
pub trait WeightInfo {
	fn claim_reward_for(r: u32, ) -> Weight;
}

/// Weights for `pallet_distribution` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Distribution::Spends` (r:1 w:1)
	/// Proof: `Distribution::Spends` (`max_values`: None, `max_size`: Some(94), added: 2569, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(355), added: 2830, mode: `MaxEncodedLen`)
	/// Storage: `Distribution::CounterForSpends` (r:1 w:1)
	/// Proof: `Distribution::CounterForSpends` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 50]`.
	fn claim_reward_for(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `864 + r * (11 ±0)`
		//  Estimated: `6196`
		// Minimum execution time: 111_649_000 picoseconds.
		Weight::from_parts(115_740_782, 6196)
			// Standard Error: 22_665
			.saturating_add(Weight::from_parts(467_124, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Distribution::Spends` (r:1 w:1)
	/// Proof: `Distribution::Spends` (`max_values`: None, `max_size`: Some(94), added: 2569, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(355), added: 2830, mode: `MaxEncodedLen`)
	/// Storage: `Distribution::CounterForSpends` (r:1 w:1)
	/// Proof: `Distribution::CounterForSpends` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 50]`.
	fn claim_reward_for(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `864 + r * (11 ±0)`
		//  Estimated: `6196`
		// Minimum execution time: 111_649_000 picoseconds.
		Weight::from_parts(115_740_782, 6196)
			// Standard Error: 22_665
			.saturating_add(Weight::from_parts(467_124, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}