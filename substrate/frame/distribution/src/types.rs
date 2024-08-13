pub use super::*;

pub use frame_support::{
	pallet_prelude::*,
	traits::{
		fungible,
		fungible::{Inspect, Mutate, MutateHold},
		fungibles,
		tokens::{Precision, Preservation},
		DefensiveOption, EnsureOrigin,
	},
	PalletId, Serialize,
};
pub use frame_system::{pallet_prelude::*, RawOrigin};
pub use scale_info::prelude::vec::Vec;
pub use sp_runtime::traits::{
	AccountIdConversion, BlockNumberProvider, Convert, Saturating, StaticLookup, Zero,
};

pub type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
/// A reward index.
pub type SpendIndex = u32;

pub type ProjectId<T> = AccountIdOf<T>;

/// The state of the payment claim.
#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo, Default)]
pub enum SpendState {
	/// Unclaimed
	#[default]
	Unclaimed,
	/// Claimed & Paid.
	Completed,
	/// Claimed but Failed.
	Failed,
}

//Processed Reward status
#[derive(Encode, Decode, Clone, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct SpendInfo<T: Config> {
	/// The asset amount of the spend.
	pub amount: BalanceOf<T>,
	/// The block number from which the spend can be claimed(24h after SpendStatus Creation).
	pub valid_from: BlockNumberFor<T>,
	/// The status of the payout/claim.
	pub status: SpendState,
	/// Corresponding project id
	pub whitelisted_project: Option<AccountIdOf<T>>,
	/// Has it been claimed?
	pub claimed: bool,
}

impl<T: Config> SpendInfo<T> {
	pub fn new(whitelisted: ProjectInfo<T>) -> Self {
		let amount = whitelisted.amount;
		let whitelisted_project = Some(whitelisted.project_account);
		let claimed = false;
		let status = SpendState::default();
		let valid_from =
			<frame_system::Pallet<T>>::block_number().saturating_add(T::BufferPeriod::get());

		let spend = SpendInfo { amount, valid_from, status, whitelisted_project, claimed };

		// Get the Spend index
		let index = SpendsCount::<T>::get();
		//Add it to the Spends storage
		Spends::<T>::insert(index, spend.clone());
		let new_index = index.checked_add(1).expect("Failed Operation");
		SpendsCount::<T>::put(new_index);

		spend
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ProjectInfo<T: Config> {
	/// AcountId that will receive the payment.
	pub project_account: ProjectId<T>,

	/// Block at which the project was submitted for reward distribution
	pub submission_block: BlockNumberFor<T>,

	/// Amount to be lock & pay for this project
	pub amount: BalanceOf<T>,
}
