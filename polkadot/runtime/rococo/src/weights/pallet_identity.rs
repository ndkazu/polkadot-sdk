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

//! Autogenerated weights for `pallet_identity`
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
// --pallet=pallet_identity
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

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 7_673_000 picoseconds.
		Weight::from_parts(8_351_866, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_302
			.saturating_add(Weight::from_parts(79_198, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn set_identity(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6978 + r * (5 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 111_646_000 picoseconds.
		Weight::from_parts(113_254_991, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 6_611
			.saturating_add(Weight::from_parts(162_119, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `11037 + s * (2589 ±0)`
		// Minimum execution time: 8_010_000 picoseconds.
		Weight::from_parts(19_868_412, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 5_018
			.saturating_add(Weight::from_parts(3_115_007, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2589).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `194 + p * (32 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 8_111_000 picoseconds.
		Weight::from_parts(19_482_392, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 3_156
			.saturating_add(Weight::from_parts(1_305_890, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7070 + r * (5 ±0) + s * (32 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 54_107_000 picoseconds.
		Weight::from_parts(56_347_715, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 10_944
			.saturating_add(Weight::from_parts(191_321, 0).saturating_mul(r.into()))
			// Standard Error: 2_135
			.saturating_add(Weight::from_parts(1_295_872, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn request_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6968 + r * (57 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 75_780_000 picoseconds.
		Weight::from_parts(76_869_773, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 5_456
			.saturating_add(Weight::from_parts(135_316, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn cancel_request(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6999`
		//  Estimated: `11037`
		// Minimum execution time: 75_769_000 picoseconds.
		Weight::from_parts(76_805_143, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 3_598
			.saturating_add(Weight::from_parts(84_593, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 5_357_000 picoseconds.
		Weight::from_parts(5_732_132, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 927
			.saturating_add(Weight::from_parts(70_832, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 5_484_000 picoseconds.
		Weight::from_parts(5_892_704, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 947
			.saturating_add(Weight::from_parts(71_231, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 5_310_000 picoseconds.
		Weight::from_parts(5_766_651, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 916
			.saturating_add(Weight::from_parts(74_776, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn provide_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7046 + r * (57 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 98_200_000 picoseconds.
		Weight::from_parts(100_105_482, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 6_152
			.saturating_add(Weight::from_parts(58_906, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7277 + r * (5 ±0) + s * (32 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 64_647_000 picoseconds.
		Weight::from_parts(68_877_027, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 9_965
			.saturating_add(Weight::from_parts(135_044, 0).saturating_mul(r.into()))
			// Standard Error: 1_944
			.saturating_add(Weight::from_parts(1_388_151, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `475 + s * (36 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 23_550_000 picoseconds.
		Weight::from_parts(29_439_842, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 1_453
			.saturating_add(Weight::from_parts(96_324, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `591 + s * (3 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 13_704_000 picoseconds.
		Weight::from_parts(15_241_441, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 498
			.saturating_add(Weight::from_parts(40_973, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `638 + s * (35 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 29_310_000 picoseconds.
		Weight::from_parts(31_712_666, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 967
			.saturating_add(Weight::from_parts(81_250, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `704 + s * (37 ±0)`
		//  Estimated: `6723`
		// Minimum execution time: 22_906_000 picoseconds.
		Weight::from_parts(24_638_729, 0)
			.saturating_add(Weight::from_parts(0, 6723))
			// Standard Error: 645
			.saturating_add(Weight::from_parts(75_121, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:0 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn add_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_056_000 picoseconds.
		Weight::from_parts(6_349_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3517`
		// Minimum execution time: 9_003_000 picoseconds.
		Weight::from_parts(9_276_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::PendingUsernames` (r:1 w:0)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	fn set_username_for(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `11037`
		// Minimum execution time: 64_724_000 picoseconds.
		Weight::from_parts(66_597_000, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:0 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn accept_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `11037`
		// Minimum execution time: 19_538_000 picoseconds.
		Weight::from_parts(20_204_000, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn remove_expired_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `3550`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(19_354_000, 0)
			.saturating_add(Weight::from_parts(0, 3550))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:0)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	fn set_primary_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `257`
		//  Estimated: `11037`
		// Minimum execution time: 15_298_000 picoseconds.
		Weight::from_parts(15_760_000, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	fn remove_dangling_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `98`
		//  Estimated: `11037`
		// Minimum execution time: 10_829_000 picoseconds.
		Weight::from_parts(11_113_000, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
