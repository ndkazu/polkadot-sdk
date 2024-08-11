#![cfg_attr(not(feature = "std"), no_std)]

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;
pub mod functions;
mod types;
pub use pallet_distribution as Distribution;
pub use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + Distribution::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The minimum duration for which votes are locked
		#[pallet::constant]
		type VoteLockingPeriod: Get<BlockNumberFor<Self>>;

		/// The maximum number of whitelisted projects per nomination round
		#[pallet::constant]
		type MaxWhitelistedProjects: Get<u32>;

		/// Time during which it is possible to cast a vote or change an existing vote.
		/// 
		#[pallet::constant]
		type VotingPeriod: Get<BlockNumberFor<Self>>;

		#[pallet::constant]
		type TemporaryRewards: Get<BalanceOf<Self>>;
	}

	/// Number of Voting Rounds executed so far
	#[pallet::storage]
	pub type VotingRoundNumber<T:Config> = StorageValue<_,u32, ValueQuery>;

	/// Returns Infos about a Voting Round agains the Voting Round index
	#[pallet::storage]
	pub type VotingRounds<T:Config> = StorageMap<_,Twox64Concat, RoundIndex, VotingRoundInfo<T>, OptionQuery>;

	/// Returns a list of Whitelisted Project accounts
	#[pallet::storage]
	pub type WhiteListedProjectAccounts<T: Config> =
		StorageValue<_, BoundedVec<ProjectId<T>, T::MaxWhitelistedProjects>, ValueQuery>;

	/// Returns Votes Infos against (project_id, voter_id) key
	#[pallet::storage]
	pub type Votes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ProjectId<T>,
		Twox64Concat,
		AccountIdOf<T>,
		VoteInfo<T>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		/// Reward successfully claimed
		RewardsAssigned { when: BlockNumberFor<T> },

		/// User's vote successfully submitted
		VoteCasted {who: AccountIdOf<T>, when: BlockNumberFor<T>, project_id: AccountIdOf<T>},
		
		/// User's vote successfully removed
		VoteRemoved {who: AccountIdOf<T>, when: BlockNumberFor<T>, project_id: AccountIdOf<T>},
		
		/// Project removed from whitelisted projects list
		ProjectUnlisted {when: BlockNumberFor<T>, project_id: AccountIdOf<T>},

		/// Project Funding Accepted by voters 
		ProjectFundingAccepted { project_id: AccountIdOf<T>, when: BlockNumberFor<T>, round_number: u32, amount: BalanceOf<T>},
		
		/// Project Funding rejected by voters
		ProjectFundingRejected { when: BlockNumberFor<T>, project_id: AccountIdOf<T> },
		
		/// A new voting round started
		VotingRoundStarted {when: BlockNumberFor<T>, round_number: u32},
				
		/// The users voting period ended. Reward calculation will start. 
		VoteActionLocked {when: BlockNumberFor<T>, round_number: u32},

		/// The voting round ended
		VotingRoundEnded {when: BlockNumberFor<T>, round_number: u32},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// This account is not connected to any WhiteListed Project.
		NotWhitelistedProject,

		/// There are no whitelisted project
		NoWhitelistedProject,

		/// The voting action failed.
		VoteFailed,

		/// No such voting data
		NoVoteData,

		/// An invalid result  was returned
		InvalidResult,

		/// Maximum number of projects submission for distribution as been reached
		MaximumProjectsNumber,

		/// This voting round does not exists
		NoRoundFound,

		/// Voting period closed for this round
		VotePeriodClosed,

		/// Not enough funds to vote, you need to decrease your stake
		NotEnoughFunds,

		/// The reward calculation failed due to an internal error
		FailedRewardCalculation,

		/// Voting round is over
		VotingRoundOver,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		
		/// Weight: see `begin_block`
		fn on_idle(n: BlockNumberFor<T>, remaining_weight: Weight) -> Weight {
			Self::on_idle_function(n,remaining_weight)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {


		/// OPF voting logic
        ///
        /// ## Dispatch Origin
        ///
        /// Must be signed
        ///
        /// ## Details
        ///
        /// This extrinsic allows users to [vote for/nominate] a whitelisted project using their funds.
        /// As a first implementation, the `conviction` parameter was not included for simplicity, but /// should be in the next iteration of the pallet.
        /// The amount defined by the user is locked and released only when the project reward is /// sent for distribution, or when the project is not dimmed fundable.
        /// Users can edit an existing vote within the vote-casting period.
        /// Then, during the vote-locked period, rewards are calculated based on the total user amount 
        /// attributed to each project by the user’s votes.
        ///
        /// ### Parameters
        /// - `project_account`: The account that will receive the reward.
        /// - `amount`: Amount that will be locked in user’s balance to nominate a project.
        /// - `is_fund`: Parameter that defines if user’s vote is in favor (*true*), or against (*false*)
        /// the project funding.
         
        /// ### Errors
        /// - [`Error::<T>::NotEnoughFunds`]: The user does not have enough balance to cast a vote
        ///  
        /// ## Events
		#[pallet::call_index(0)]
		pub fn vote(origin: OriginFor<T>, project_account: ProjectId<T>, amount: BalanceOf<T>, is_fund: bool) -> DispatchResult {
			let voter = ensure_signed(origin)?;
			// Get current voting round & check if we are in voting period or not
			Self::period_check()?;
			// Check that voter has enough funds to vote
			let voter_balance = T::NativeBalance::total_balance(&voter);
			ensure!(voter_balance>amount, Error::<T>::NotEnoughFunds);
			let mut voter_holds = BalanceOf::<T>::zero();
			
			let all_votes = Votes::<T>::iter();
			for vote in all_votes{
				if vote.0 != project_account.clone() && vote.1 == voter.clone(){
					voter_holds.saturating_add(vote.2.amount);
				} 
			}
			let available_funds = voter_balance.saturating_sub(voter_holds);
			ensure!(available_funds > amount, Error::<T>::NotEnoughFunds);

			// Vote action executed

			Self::try_vote(voter.clone(),project_account.clone(),amount,is_fund)?;

			let when = T::BlockNumberProvider::current_block_number();
			Self::deposit_event(Event::<T>::VoteCasted{
			who: voter,
			when,
			project_id: project_account,
		});

			Ok(())
		}


		/// OPF vote removal logic
        ///
        /// ## Dispatch Origin
        ///
        /// Must be signed
        ///
        /// ## Details
        ///
        /// This extrinsic allows users to remove a casted vote, as long as it is within the vote-casting period.
        ///
        /// ### Parameters
        /// - `project_account`: The account that will receive the reward.
        ///
        /// ### Errors
        /// - [`Error::<T>::NotEnoughFunds`]: The user does not have enough balance to cast a vote
        ///  
        /// ## Events
		#[pallet::call_index(1)]
		pub fn remove_vote(origin: OriginFor<T>, project_account: ProjectId<T>) -> DispatchResult {
			let voter = ensure_signed(origin)?;
			// Get current voting round & check if we are in voting period or not
			Self::period_check()?;
			// Removal action executed
			Self::try_remove_vote(voter.clone(),project_account.clone())?;

			let when = T::BlockNumberProvider::current_block_number();
			Self::deposit_event(Event::<T>::VoteRemoved{
				who: voter,
				when,
				project_id: project_account, 
			});

			Ok(())
		}
	}
}
