pub use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::OnIdle;

pub fn next_block() {
	System::set_block_number(System::block_number() + 1);
	AllPalletsWithSystem::on_initialize(System::block_number());
    AllPalletsWithSystem::on_idle(System::block_number(), Weight::MAX);
}

pub fn run_to_block(n: BlockNumberFor<Test>) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			AllPalletsWithSystem::on_finalize(System::block_number());
		}
		next_block();
	}
}

pub fn create_project_list(){
    const max_number:u64 = <Test as Config>::MaxWhitelistedProjects::get() as u64;
    let mut bounded_vec = BoundedVec::<u64, <Test as Config>::MaxWhitelistedProjects>::new();
    for i in 0..max_number {
        let _= bounded_vec.try_push(i+100);
        
    }
    WhiteListedProjectAccounts::<Test>::mutate(|value|{
        *value = bounded_vec;
    });
    
}

#[test]
fn vote_works() {
    new_test_ext().execute_with(|| {

        //create whitelisted projects list
        create_project_list();

    })
}