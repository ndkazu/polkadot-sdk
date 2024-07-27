
#![cfg_attr(not(feature = "std"), no_std)]



// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;
mod types;
pub use types::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

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
		type PotId: Get<PalletId>;

		/// Tokens Existential deposit
		type Existential: Get<BalanceOf<Self>>;

		type RuntimeHoldReason: From<HoldReason>;

		// This the minimum required time period between project whitelisting
		// and payment/reward_claim from the treasury.
		type PaymentPeriod: Get<BlockNumberFor<Self>>;
		
		type MaxProjects: Get<u32>;


	}
	
	/// A reason for placing a hold on funds.
	#[pallet::composite_enum]
	pub enum HoldReason {
		/// Funds are held to register for free transactions.
		#[codec(index = 0)]
		ProposalBond,
	}

	#[pallet::storage]
	pub type Something<T> = StorageValue<_, u32>;

	/// Number of spendings that have been executed so far.
	#[pallet::storage]
	pub type SpendingsCount<T: Config> = StorageValue<_, SpendingIndex, ValueQuery>;

	/// Executed spendings information.
	#[pallet::storage]
	pub type CompletedSpendings<T: Config> =
		StorageMap<_, Twox64Concat, SpendingIndex, SpendingInfo<T>, OptionQuery>;

	/// Spendings that still have to be completed.
	#[pallet::storage]
	pub type Spendings<T: Config> =
		StorageMap<_, Twox64Concat, SpendingIndex, SpendingInfo<T>, OptionQuery>;

	/// List or whitelisted projects to be rewarded
	#[pallet::storage]
	pub type Projects<T: Config> = 
		StorageValue<_, BoundedVec<ProjectInfo<T>, T::MaxProjects>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// We usually use passive tense for events.
		SomethingStored { something: u32, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::call_index(0)]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::<T>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });

			// Return a successful `DispatchResult`
			Ok(())
		}

	}




	
}