
#![cfg_attr(not(feature = "std"), no_std)]



// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;
mod types;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		/// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type to access the Balances Pallet.
		type NativeBalance: fungible::Inspect<Self::AccountId>
			+ fungible::Mutate<Self::AccountId>
			+ fungible::hold::Inspect<Self::AccountId>
			+ fungible::hold::Mutate<Self::AccountId, Reason = Self::RuntimeHoldReason>
			+ fungible::freeze::Inspect<Self::AccountId>
			+ fungible::freeze::Mutate<Self::AccountId>;

		/// Treasury account Id
		type TreasuryAccount: Get<PalletId>;

		/// Tokens Existential deposit
		type Existential: Get<BalanceOf<Self>>;

		/// Tokens Existential deposit
		type ProposalBond: Get<BalanceOf<Self>>;

		/// Time interval to check the status of SpendingProposals & SpendingStatus
		type SpendCheck: Get<BlockNumberFor<Self>>;

		type RuntimeHoldReason: From<HoldReason>;

		type PaymentPeriod: Get<BlockNumberFor<Self>>;


	}
	
	/// A reason for the pallet placing a hold on funds.
	#[pallet::composite_enum]
	pub enum HoldReason {
		/// Funds are held to register for free transactions.
		#[codec(index = 0)]
		ProposalBond,
	}
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// We usually use passive tense for events.
		SomethingStored { something: u32, who: T::AccountId },
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
}