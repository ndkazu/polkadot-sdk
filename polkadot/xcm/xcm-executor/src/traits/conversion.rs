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

use core::{marker::PhantomData, result::Result};
use frame_support::traits::{Contains, OriginTrait};
use sp_runtime::{traits::Dispatchable, DispatchErrorWithPostInfo};
use xcm::latest::prelude::*;

/// Means of converting a location into an account identifier.
pub trait ConvertLocation<AccountId> {
	/// Convert the `location` into `Some` account ID, or `None` if not possible.
	fn convert_location(location: &Location) -> Option<AccountId>;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl<AccountId> ConvertLocation<AccountId> for Tuple {
	fn convert_location(l: &Location) -> Option<AccountId> {
		for_tuples!( #(
			match Tuple::convert_location(l) {
				Some(result) => return Some(result),
				None => {},
			}
		)* );
		None
	}
}

/// A converter `trait` for origin types.
///
/// Can be amalgamated into tuples. If any of the tuple elements returns `Ok(_)`, it short circuits.
/// Else, the `Err(_)` of the last tuple item is returned. Each intermediate `Err(_)` might return a
/// different `origin` of type `Origin` which is passed to the next convert item.
///
/// ```rust
/// # use xcm::latest::{Location, Junctions, Junction, OriginKind};
/// # use staging_xcm_executor::traits::ConvertOrigin;
/// // A convertor that will bump the para id and pass it to the next one.
/// struct BumpParaId;
/// impl ConvertOrigin<u32> for BumpParaId {
/// 	fn convert_origin(origin: impl Into<Location>, _: OriginKind) -> Result<u32, Location> {
/// 		match origin.into().unpack() {
/// 			(0, [Junction::Parachain(id)]) => {
/// 				Err([Junction::Parachain(id + 1)].into())
/// 			}
/// 			_ => unreachable!()
/// 		}
/// 	}
/// }
///
/// struct AcceptPara7;
/// impl ConvertOrigin<u32> for AcceptPara7 {
/// 	fn convert_origin(origin: impl Into<Location>, _: OriginKind) -> Result<u32, Location> {
///         let origin = origin.into();
/// 		match origin.unpack() {
/// 			(0, [Junction::Parachain(id)]) if *id == 7 => {
/// 				Ok(7)
/// 			}
/// 			_ => Err(origin)
/// 		}
/// 	}
/// }
/// # fn main() {
/// let origin: Location = [Junction::Parachain(6)].into();
/// assert!(
/// 	<(BumpParaId, AcceptPara7) as ConvertOrigin<u32>>::convert_origin(origin, OriginKind::Native)
/// 		.is_ok()
/// );
/// # }
/// ```
pub trait ConvertOrigin<Origin> {
	/// Attempt to convert `origin` to the generic `Origin` whilst consuming it.
	fn convert_origin(origin: impl Into<Location>, kind: OriginKind) -> Result<Origin, Location>;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl<O> ConvertOrigin<O> for Tuple {
	fn convert_origin(origin: impl Into<Location>, kind: OriginKind) -> Result<O, Location> {
		let origin = origin.into();

		tracing::trace!(
			target: "xcm::convert_origin",
			?origin,
			?kind,
			"Converting origin",
		);

		for_tuples!( #(
			let convert_origin = core::any::type_name::<Tuple>();

			let origin = match Tuple::convert_origin(origin, kind) {
				Err(o) => {
					tracing::trace!(
						target: "xcm::convert_origin",
						%convert_origin,
						"Convert origin step failed",
					);

					o
				},
				Ok(o) => {
					tracing::trace!(
						target: "xcm::convert_origin",
						%convert_origin,
						"Convert origin step succeeded",
					);

					return Ok(o)
				}
			};
		)* );

		tracing::trace!(
			target: "xcm::convert_origin",
			"Converting origin failed",
		);

		Err(origin)
	}
}

/// Defines how a call is dispatched with given origin.
/// Allows to customize call dispatch, such as adapting the origin based on the call
/// or modifying the call.
pub trait CallDispatcher<Call: Dispatchable> {
	fn dispatch(
		call: Call,
		origin: Call::RuntimeOrigin,
	) -> Result<Call::PostInfo, DispatchErrorWithPostInfo<Call::PostInfo>>;
}

pub struct WithOriginFilter<Filter>(PhantomData<Filter>);
impl<Call, Filter> CallDispatcher<Call> for WithOriginFilter<Filter>
where
	Call: Dispatchable,
	Call::RuntimeOrigin: OriginTrait,
	<<Call as Dispatchable>::RuntimeOrigin as OriginTrait>::Call: 'static,
	Filter: Contains<<<Call as Dispatchable>::RuntimeOrigin as OriginTrait>::Call> + 'static,
{
	fn dispatch(
		call: Call,
		mut origin: <Call as Dispatchable>::RuntimeOrigin,
	) -> Result<
		<Call as Dispatchable>::PostInfo,
		DispatchErrorWithPostInfo<<Call as Dispatchable>::PostInfo>,
	> {
		origin.add_filter(Filter::contains);
		call.dispatch(origin)
	}
}

// We implement it for every calls so they can dispatch themselves
// (without any change).
impl<Call: Dispatchable> CallDispatcher<Call> for Call {
	fn dispatch(
		call: Call,
		origin: Call::RuntimeOrigin,
	) -> Result<Call::PostInfo, DispatchErrorWithPostInfo<Call::PostInfo>> {
		call.dispatch(origin)
	}
}
