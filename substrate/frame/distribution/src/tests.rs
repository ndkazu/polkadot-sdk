pub use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

pub fn next_block() {
	System::set_block_number(<Test as Config>::BlockNumberProvider::current_block_number() + 1);
	AllPalletsWithSystem::on_initialize(
		<Test as Config>::BlockNumberProvider::current_block_number(),
	);
}

pub fn run_to_block(n: BlockNumberFor<Test>) {
	while <Test as Config>::BlockNumberProvider::current_block_number() < n {
		if <Test as Config>::BlockNumberProvider::current_block_number() > 1 {
			AllPalletsWithSystem::on_finalize(
				<Test as Config>::BlockNumberProvider::current_block_number(),
			);
		}
		next_block();
	}
}

pub fn create_project(project_account: AccountId, amount: u128) {
	let submission_block = <Test as Config>::BlockNumberProvider::current_block_number();
	let project: types::ProjectInfo<Test> =
		ProjectInfo { project_account, submission_block, amount };
	Projects::<Test>::mutate(|value| {
		let mut val = value.clone();
		let _ = val.try_push(project);
		*value = val;
	});
}

#[test]
fn spends_creation_works() {
	new_test_ext().execute_with(|| {
		// Add 3 projects
		let amount1 = 1_000_000 * BSX;
		let amount2 = 1_200_000 * BSX;
		let amount3 = 2_000_000 * BSX;
		create_project(ALICE, amount1);
		create_project(BOB, amount2);
		create_project(DAVE, amount3);

		// The Spends Storage should be empty
		assert_eq!(SpendsCount::<Test>::get(), 0);

		// Move to epoch block => Warning: We set the system block at 1 in mock.rs, so now =
		// Epoch_Block + 1
		let now = <Test as Config>::BlockNumberProvider::current_block_number()
			.saturating_add(<Test as Config>::EpochDurationBlocks::get().into());
		run_to_block(now);

		// We should have 3 Spends
		assert!(SpendsCount::<Test>::get() == 3);

		// The 3 Spends are known
		let alice_spend: types::SpendInfo<Test> = SpendInfo {
			amount: amount1,
			valid_from: now,
			status: types::SpendState::default(),
			whitelisted_project: Some(ALICE),
			claimed: false,
		};

		let bob_spend: types::SpendInfo<Test> = SpendInfo {
			amount: amount2,
			valid_from: now,
			status: types::SpendState::default(),
			whitelisted_project: Some(BOB),
			claimed: false,
		};

		let dave_spend: types::SpendInfo<Test> = SpendInfo {
			amount: amount3,
			valid_from: now,
			status: types::SpendState::default(),
			whitelisted_project: Some(DAVE),
			claimed: false,
		};

		// List of Spends actually created & stored
		let list0: Vec<_> = Spends::<Test>::iter_keys().collect();
		let list: Vec<_> = list0.into_iter().map(|x| Spends::<Test>::get(x)).collect();

		expect_events(vec![
			RuntimeEvent::Distribution(Event::SpendCreated {
				when: now.saturating_sub(1),
				amount: list[0].clone().unwrap().amount,
				project_account: list[0].clone().unwrap().whitelisted_project.unwrap(),
			}),
			RuntimeEvent::Distribution(Event::SpendCreated {
				when: now.saturating_sub(1),
				amount: list[1].clone().unwrap().amount,
				project_account: list[1].clone().unwrap().whitelisted_project.unwrap(),
			}),
			RuntimeEvent::Distribution(Event::SpendCreated {
				when: now.saturating_sub(1),
				amount: list[2].clone().unwrap().amount,
				project_account: list[2].clone().unwrap().whitelisted_project.unwrap(),
			}),
		]);

		assert!(list.contains(&Some(alice_spend)));
		assert!(list.contains(&Some(bob_spend)));
		assert!(list.contains(&Some(dave_spend)));
	})
}

#[test]
fn funds_are_locked() {
	new_test_ext().execute_with(|| {
		// Add 3 projects
		let amount1 = 1_000_000 * BSX;
		let amount2 = 1_200_000 * BSX;
		let amount3 = 2_000_000 * BSX;
		create_project(ALICE, amount1);
		create_project(BOB, amount2);
		create_project(DAVE, amount3);

		// The Spends Storage should be empty
		assert_eq!(SpendsCount::<Test>::get(), 0);

		// Move to epoch block => Warning: We set the system block at 1 in mock.rs, so now =
		// Epoch_Block + 1
		let now = <Test as Config>::BlockNumberProvider::current_block_number()
			.saturating_add(<Test as Config>::EpochDurationBlocks::get().into());
		run_to_block(now);

		let total_on_hold = amount1.saturating_add(amount2).saturating_add(amount3);
		let pot_account = Distribution::pot_account();
		let hold =
			<<Test as Config>::NativeBalance as fungible::hold::Inspect<u64>>::balance_on_hold(
				&HoldReason::FundsReserved.into(),
				&pot_account,
			);
		assert_eq!(total_on_hold, hold);
	})
}

#[test]
fn funds_claim_works() {
	new_test_ext().execute_with(|| {
		// Add 3 projects
		let amount1 = 1_000_000 * BSX;
		let amount2 = 1_200_000 * BSX;
		let amount3 = 2_000_000 * BSX;
		create_project(ALICE, amount1);
		create_project(BOB, amount2);
		create_project(DAVE, amount3);

		// The Spends Storage should be empty
		assert_eq!(SpendsCount::<Test>::get(), 0);

		assert_eq!(Projects::<Test>::get().len(), 3);

		// Move to epoch block => Warning: We set the system block at 1 in mock.rs, so now =
		// Epoch_Block + 1
		let mut now = <Test as Config>::BlockNumberProvider::current_block_number()
			.saturating_add(<Test as Config>::EpochDurationBlocks::get().into());
		run_to_block(now);

		println!("the first mystery block is:{:?}", now);
		let project = Spends::<Test>::get(0).unwrap();
		let project_account = project.whitelisted_project.unwrap();
		let balance_0 =
			<<Test as Config>::NativeBalance as fungible::Inspect<u64>>::balance(&project_account);
		now = now.saturating_add(project.valid_from);
		run_to_block(now);

		println!("the mystery block is:{:?}", now);
		assert_ok!(Distribution::claim_reward_for(
			RawOrigin::Signed(EVE).into(),
			project_account.clone(),
		));
		let balance_1 =
			<<Test as Config>::NativeBalance as fungible::Inspect<u64>>::balance(&project_account);

		assert!(balance_1 > balance_0);
		assert_eq!(Projects::<Test>::get().len(), 0);
	})
}

#[test]
fn funds_claim_fails_before_claim_period() {
	new_test_ext().execute_with(|| {
		// Add 3 projects
		let amount1 = 1_000_000 * BSX;
		let amount2 = 1_200_000 * BSX;
		let amount3 = 2_000_000 * BSX;
		create_project(ALICE, amount1);
		create_project(BOB, amount2);
		create_project(DAVE, amount3);

		// The Spends Storage should be empty
		assert_eq!(SpendsCount::<Test>::get(), 0);

		// Move to epoch block => Warning: We set the system block at 1 in mock.rs, so now =
		// Epoch_Block + 1
		let now = <Test as Config>::BlockNumberProvider::current_block_number()
			.saturating_add(<Test as Config>::EpochDurationBlocks::get().into());
		run_to_block(now);

		let project = Spends::<Test>::get(0).unwrap();
		let project_account = project.whitelisted_project.unwrap();

		assert_noop!(
			Distribution::claim_reward_for(RawOrigin::Signed(EVE).into(), project_account.clone(),),
			Error::<Test>::NotClaimingPeriod
		);
	})
}
