pub use super::*;
impl<T: Config> Pallet<T> {
	pub fn pot_account() -> AccountIdOf<T> {
		// Get Pot account
		let pot_id = T::PotId::get();
		let pot_account: AccountIdOf<T> = pot_id.into_account_truncating();
		pot_account
	}

	pub fn get_spending(project_account: AccountIdOf<T>) -> Vec<SpendingIndex> {
		let mut spendings: Vec<SpendingIndex> = Vec::new();
		let value = Some(project_account);

		for spending in Spendings::<T>::iter() {
			let info = spending.1;
			if info.whitelisted_project == value {
				spendings.push(spending.0);
			}
		}

		spendings
	}

	/// Series of checks on the Pot, to ensure that we have enough funds
	/// before executing a spending
	pub fn pot_check(amount: BalanceOf<T>) -> DispatchResult {
		// Get Pot account
		let pot_account: AccountIdOf<T> = Self::pot_account();

		// Check that the Pot as enough funds for the transfer
		let balance = T::NativeBalance::balance(&pot_account);
		let minimum_balance = T::NativeBalance::minimum_balance();
		let remaining_balance = balance.saturating_sub(amount);

		ensure!(remaining_balance > minimum_balance, Error::<T>::InsufficientPotReserves);
		ensure!(balance > amount, Error::<T>::InsufficientPotReserves);
		Ok(())
	}

	/// Funds transfer from the Pot to a project account
	pub fn spending(
		amount: BalanceOf<T>,
		beneficiary: AccountIdOf<T>,
		spending_index: u32,
	) -> DispatchResult {
		// Get Pot account
		let pot_account: AccountIdOf<T> = Self::pot_account();

		//Operate the transfer
		let result =
			T::NativeBalance::transfer(&pot_account, &beneficiary, amount, Preservation::Preserve)
				.map_err(|_| Error::<T>::TransferFailed);

		Self::process_failed_spending_result(spending_index, result)?;

		Ok(())
	}

	/// Helper function used to change the status of a failed spending
	pub fn process_failed_spending_result(
		spending_index: u32,
		result: Result<BalanceOf<T>, Error<T>>,
	) -> Result<BalanceOf<T>, Error<T>> {
		match result {
			Ok(x) => {
				// Change spending status
				Spendings::<T>::mutate(spending_index, |val| {
					let mut val0 = val.clone().unwrap();
					val0.status = SpendingState::Completed;
					*val = Some(val0);
				});
				Ok(x)
			},
			Err(_e) => {
				// Change spending status
				Spendings::<T>::mutate(spending_index, |val| {
					let mut val0 = val.clone().unwrap();
					val0.status = SpendingState::Failed;
					*val = Some(val0);
				});
				Err(Error::<T>::FailedSpendingOperation)
			},
		}
	}

	// ToDo in begin_block
	// At the beginning of every Epoch, populate the `Spendings` storage from the `Projects` storage (populated by an external process/pallet)
	// make sure that there is enough funds before creating a new `SpendingInfo`, and `ProjectInfo`
	// corresponding to a created `SpendingInfo` should be removed from the `Projects` storage.
	// This is also a good place to Reserve the funds for created `SpendingInfos`.
	// the function will be use in a hook.

	pub fn begin_block(now: BlockNumberFor<T>) -> Weight {
		let max_block_weight = Weight::from_parts(1000_u64, 0);
		let epoch = T::EpochDurationBlocks::get();

		//We reach the check period
		if (now % epoch).is_zero() {
			let mut projects = Projects::<T>::get();

			if projects.len() > 0 {
				for project in projects.clone() {
					// check if the pot has enough fund for the spending
					let check = Self::pot_check(project.amount);
					let _result = match check {
						Ok(x) => {
							// Create a new spending
							let new_spending = SpendingInfo::<T>::new(project.clone());

							// Reserve funds for the project
							let pot = Self::pot_account();
							let _ = T::NativeBalance::hold(
								&HoldReason::FundsReserved.into(),
								&pot,
								project.amount,
							)
							.map_err(|_| Error::<T>::FundsReserveFailed);

							// Remove project from project_list
							projects.retain(|value| *value != project);

							// Emmit an event
							let now = <frame_system::Pallet<T>>::block_number();
							Self::deposit_event(Event::SpendingCreated {
								when: now,
								amount: new_spending.amount,
								project_account: project.project_account,
							});

							Ok(x)
						},
						Err(_e) => Err(Error::<T>::InsufficientPotReserves),
					};
				}
			}

			// Update project storage
			Projects::<T>::mutate(|val| {
				*val = projects;
			});
		}
		max_block_weight
	}
}
