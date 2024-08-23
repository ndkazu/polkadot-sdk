// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Helper functions for Distribution pallet.

pub use super::*;
impl<T: Config> Pallet<T> {
	pub fn pot_account() -> AccountIdOf<T> {
		// Get Pot account
		let pot_id = T::PotId::get();
		let pot_account: AccountIdOf<T> = pot_id.into_account_truncating();
		pot_account
	}

	pub fn get_spend(project_account: ProjectId<T>) -> Vec<SpendIndex> {
		let mut spends: Vec<SpendIndex> = Vec::new();
		let value = Some(project_account);

		for (index,info) in Spends::<T>::iter() {
			if info.whitelisted_project == value {
				spends.push(index);
			}
		}

		spends
	}

	/// Series of checks on the Pot, to ensure that we have enough funds
	/// before executing a Spend
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
	pub fn spend(
		amount: BalanceOf<T>,
		beneficiary: AccountIdOf<T>,
		spend_index: u32,
	) -> DispatchResult {
		// Get Pot account
		let pot_account: AccountIdOf<T> = Self::pot_account();

		//Operate the transfer
		let result =
			T::NativeBalance::transfer(&pot_account, &beneficiary, amount, Preservation::Preserve)
				.map_err(|_| Error::<T>::TransferFailed);

		Self::process_failed_spend_result(spend_index, result)?;

		Ok(())
	}

	/// Helper function used to change the status of a failed Spend
	/// As we reserve the funds in the pot before doing a transfer 
	/// the probability of a transaction failing is very low.
	/// However, an additionnal fail safe won't hurt.
	pub fn process_failed_spend_result(
		spend_index: u32,
		result: Result<BalanceOf<T>, Error<T>>,
	) -> Result<BalanceOf<T>, Error<T>> {
		match result {
			Ok(x) => {
				// Change Spend status
				Spends::<T>::mutate(spend_index, |val| {
					let mut val0 = val.clone().unwrap();
					val0.status = SpendState::Completed;
					*val = Some(val0);
				});
				Ok(x)
			},
			Err(_e) => {
				// Change Spend status
				Spends::<T>::mutate(spend_index, |val| {
					let mut val0 = val.clone().unwrap();
					val0.status = SpendState::Failed;
					*val = Some(val0);
				});
				Err(Error::<T>::FailedSpendOperation)
			},
		}
	}

	// Done in begin_block
	// At the beginning of every Epoch, populate the `Spends` storage from the `Projects` storage
	// (populated by an external process/pallet) make sure that there is enough funds before
	// creating a new `SpendInfo`, and `ProjectInfo` corresponding to a created `SpendInfo`
	// should be removed from the `Projects` storage. This is also a good place to Reserve the
	// funds for created `SpendInfos`. the function will be use in a hook.

	pub fn begin_block(now: BlockNumberFor<T>) -> Weight {
		let max_block_weight = Weight::from_parts(1000_u64, 0);
		let epoch = T::EpochDurationBlocks::get();

		//We reach the check period
		if (now % epoch).is_zero() {
			let mut projects = Projects::<T>::get();

			if projects.len() > 0 {
				for project in projects.clone() {
					// check if the pot has enough fund for the Spend
					let check = Self::pot_check(project.amount);
					let _result = match check {
						Ok(x) => {
							// Create a new Spend
							let new_spend = SpendInfo::<T>::new(project.clone());

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
							let now = T::BlockNumberProvider::current_block_number();
							Self::deposit_event(Event::SpendCreated {
								when: now,
								amount: new_spend.amount,
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