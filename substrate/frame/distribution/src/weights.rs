
//! Autogenerated weights for `pallet_distribution`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
	/// Storage: `Distribution::Spends` (r:51 w:1)
	/// Proof: `Distribution::Spends` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 50]`.
	fn claim_reward_for(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `595 + r * (75 ±0)`
		//  Estimated: `6196 + r * (2542 ±0)`
		// Minimum execution time: 118_211_000 picoseconds.
		Weight::from_parts(113_390_031, 6196)
			// Standard Error: 20_848
			.saturating_add(Weight::from_parts(4_798_536, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 2542).saturating_mul(r.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Distribution::Spends` (r:51 w:1)
	/// Proof: `Distribution::Spends` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 50]`.
	fn claim_reward_for(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `595 + r * (75 ±0)`
		//  Estimated: `6196 + r * (2542 ±0)`
		// Minimum execution time: 118_211_000 picoseconds.
		Weight::from_parts(113_390_031, 6196)
			// Standard Error: 20_848
			.saturating_add(Weight::from_parts(4_798_536, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 2542).saturating_mul(r.into()))
	}
}
