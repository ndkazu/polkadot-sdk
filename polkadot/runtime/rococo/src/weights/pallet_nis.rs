// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_nis`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-bn-ce5rx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("rococo-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_nis
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_nis`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nis::WeightInfo for WeightInfo<T> {
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 999]`.
	fn place_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6209 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 39_592_000 picoseconds.
		Weight::from_parts(38_234_037, 0)
			.saturating_add(Weight::from_parts(0, 51487))
			// Standard Error: 1_237
			.saturating_add(Weight::from_parts(88_816, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn place_bid_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `54211`
		//  Estimated: `51487`
		// Minimum execution time: 134_847_000 picoseconds.
		Weight::from_parts(139_510_000, 0)
			.saturating_add(Weight::from_parts(0, 51487))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn retract_bid(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6209 + l * (48 ±0)`
		//  Estimated: `51487`
		// Minimum execution time: 43_330_000 picoseconds.
		Weight::from_parts(35_097_881, 0)
			.saturating_add(Weight::from_parts(0, 51487))
			// Standard Error: 1_119
			.saturating_add(Weight::from_parts(73_640, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Nis::Summary` (r:1 w:0)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn fund_deficit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `225`
		//  Estimated: `3593`
		// Minimum execution time: 29_989_000 picoseconds.
		Weight::from_parts(30_865_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `NisCounterpartBalances::Account` (r:1 w:1)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	fn communify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `387`
		//  Estimated: `3593`
		// Minimum execution time: 58_114_000 picoseconds.
		Weight::from_parts(59_540_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `NisCounterpartBalances::Account` (r:1 w:1)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	fn privatize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `543`
		//  Estimated: `3593`
		// Minimum execution time: 75_780_000 picoseconds.
		Weight::from_parts(77_097_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(103), added: 2578, mode: `MaxEncodedLen`)
	fn thaw_private() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `387`
		//  Estimated: `3593`
		// Minimum execution time: 46_133_000 picoseconds.
		Weight::from_parts(47_250_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Nis::Receipts` (r:1 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `NisCounterpartBalances::Account` (r:1 w:1)
	/// Proof: `NisCounterpartBalances::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn thaw_communal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `488`
		//  Estimated: `3593`
		// Minimum execution time: 77_916_000 picoseconds.
		Weight::from_parts(79_427_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Nis::Summary` (r:1 w:1)
	/// Proof: `Nis::Summary` (`max_values`: Some(1), `max_size`: Some(40), added: 535, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Nis::QueueTotals` (r:1 w:1)
	/// Proof: `Nis::QueueTotals` (`max_values`: Some(1), `max_size`: Some(6002), added: 6497, mode: `MaxEncodedLen`)
	fn process_queues() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6658`
		//  Estimated: `7487`
		// Minimum execution time: 22_992_000 picoseconds.
		Weight::from_parts(24_112_000, 0)
			.saturating_add(Weight::from_parts(0, 7487))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Nis::Queues` (r:1 w:1)
	/// Proof: `Nis::Queues` (`max_values`: None, `max_size`: Some(48022), added: 50497, mode: `MaxEncodedLen`)
	fn process_queue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `51487`
		// Minimum execution time: 3_856_000 picoseconds.
		Weight::from_parts(4_125_000, 0)
			.saturating_add(Weight::from_parts(0, 51487))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Nis::Receipts` (r:0 w:1)
	/// Proof: `Nis::Receipts` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn process_bid() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_344_000 picoseconds.
		Weight::from_parts(4_545_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
