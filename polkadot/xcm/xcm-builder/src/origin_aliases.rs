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

//! Implementation for `ContainsPair<Location, Location>`.

use core::marker::PhantomData;
use frame_support::traits::{Contains, ContainsPair};
use xcm::latest::prelude::*;

/// Alias a Foreign `AccountId32` with a local `AccountId32` if the foreign `AccountId32` matches
/// the `Prefix` pattern.
///
/// Requires that the prefixed origin `AccountId32` matches the target `AccountId32`.
pub struct AliasForeignAccountId32<Prefix>(PhantomData<Prefix>);
impl<Prefix: Contains<Location>> ContainsPair<Location, Location>
	for AliasForeignAccountId32<Prefix>
{
	fn contains(origin: &Location, target: &Location) -> bool {
		if let (prefix, Some(account_id @ AccountId32 { .. })) =
			origin.clone().split_last_interior()
		{
			return Prefix::contains(&prefix) &&
				*target == Location { parents: 0, interior: [account_id].into() }
		}
		false
	}
}
