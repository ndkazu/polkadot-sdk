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

//! Autogenerated weights for `pallet_ranked_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-02-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `e0f303704c84`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --extrinsic=*
// --runtime=target/production/wbuild/collectives-westend-runtime/collectives_westend_runtime.wasm
// --pallet=pallet_ranked_collective
// --header=/__w/polkadot-sdk/polkadot-sdk/cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/collectives/collectives-westend/src/weights
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096
// --no-storage-info
// --no-min-squares
// --no-median-slopes

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_ranked_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ranked_collective::WeightInfo for WeightInfo<T> {
	/// Storage: `AmbassadorCollective::Members` (r:1 w:1)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:1)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IndexToId` (r:0 w:1)
	/// Proof: `AmbassadorCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IdToIndex` (r:0 w:1)
	/// Proof: `AmbassadorCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 16_784_000 picoseconds.
		Weight::from_parts(17_177_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorCollective::Members` (r:1 w:1)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:11 w:11)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IdToIndex` (r:11 w:22)
	/// Proof: `AmbassadorCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IndexToId` (r:11 w:22)
	/// Proof: `AmbassadorCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 10]`.
	/// The range of component `r` is `[0, 10]`.
	fn remove_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `414 + r * (213 ±0)`
		//  Estimated: `3519 + r * (2529 ±0)`
		// Minimum execution time: 36_242_000 picoseconds.
		Weight::from_parts(39_089_749, 0)
			.saturating_add(Weight::from_parts(0, 3519))
			// Standard Error: 31_470
			.saturating_add(Weight::from_parts(17_700_585, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2529).saturating_mul(r.into()))
	}
	/// Storage: `AmbassadorCollective::Members` (r:1 w:1)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:1)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IndexToId` (r:0 w:1)
	/// Proof: `AmbassadorCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IdToIndex` (r:0 w:1)
	/// Proof: `AmbassadorCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 10]`.
	/// The range of component `r` is `[0, 10]`.
	fn promote_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `214 + r * (17 ±0)`
		//  Estimated: `3507`
		// Minimum execution time: 20_363_000 picoseconds.
		Weight::from_parts(21_753_026, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			// Standard Error: 6_072
			.saturating_add(Weight::from_parts(380_671, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorCollective::Members` (r:1 w:1)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:1)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IdToIndex` (r:1 w:2)
	/// Proof: `AmbassadorCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IndexToId` (r:1 w:2)
	/// Proof: `AmbassadorCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 10]`.
	/// The range of component `r` is `[0, 10]`.
	fn demote_member(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + r * (72 ±0)`
		//  Estimated: `3519`
		// Minimum execution time: 36_630_000 picoseconds.
		Weight::from_parts(40_091_658, 0)
			.saturating_add(Weight::from_parts(0, 3519))
			// Standard Error: 23_729
			.saturating_add(Weight::from_parts(816_694, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::Voting` (r:1 w:1)
	/// Proof: `AmbassadorCollective::Voting` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `529`
		//  Estimated: `317568`
		// Minimum execution time: 50_172_000 picoseconds.
		Weight::from_parts(53_697_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::VotingCleanup` (r:1 w:0)
	/// Proof: `AmbassadorCollective::VotingCleanup` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::Voting` (r:100 w:100)
	/// Proof: `AmbassadorCollective::Voting` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	fn cleanup_poll(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `400 + n * (50 ±0)`
		//  Estimated: `4365 + n * (2540 ±0)`
		// Minimum execution time: 15_768_000 picoseconds.
		Weight::from_parts(21_127_585, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			// Standard Error: 4_931
			.saturating_add(Weight::from_parts(1_356_379, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2540).saturating_mul(n.into()))
	}
	/// Storage: `AmbassadorCollective::Members` (r:2 w:2)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:2 w:2)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IdToIndex` (r:2 w:4)
	/// Proof: `AmbassadorCollective::IdToIndex` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCore::Member` (r:2 w:2)
	/// Proof: `AmbassadorCore::Member` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCore::MemberEvidence` (r:1 w:0)
	/// Proof: `AmbassadorCore::MemberEvidence` (`max_values`: None, `max_size`: Some(65581), added: 68056, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorSalary::Claimant` (r:2 w:2)
	/// Proof: `AmbassadorSalary::Claimant` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::IndexToId` (r:0 w:2)
	/// Proof: `AmbassadorCollective::IndexToId` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	fn exchange_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `597`
		//  Estimated: `69046`
		// Minimum execution time: 84_850_000 picoseconds.
		Weight::from_parts(87_564_000, 0)
			.saturating_add(Weight::from_parts(0, 69046))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(14))
	}
}
