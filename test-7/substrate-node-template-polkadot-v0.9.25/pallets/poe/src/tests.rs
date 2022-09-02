use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok,BoundedVec};


#[test]
fn create_claim(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
		let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClinetLenght>::try_from(claim.clone()).unwrap();
		assert_eq!(Proofs::<Test>::get(&bounded_claim),
		 Some((1,frame_system::Pallet::<Test>::block_number())));
	})
}

#[test]
fn create_claim_already_exist(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		_ = PoeModule::create_claim(Origin::signed(1),claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
		let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClinetLenght>::try_from(claim.clone()).unwrap();
		assert_eq!(Proofs::<Test>::get(&bounded_claim),
				Some( (1,frame_system::Pallet::<Test>::block_number()) ) );
		assert_ok!(PoeModule::remove_claim(Origin::signed(1),claim.clone()));
		assert_eq!(Proofs::<Test>::get(&bounded_claim),None);
	})
}

#[test]
fn not_claim_owner(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
		//_ = BoundedVec::<u8,<Test as Config>::MaxClinetLenght>::try_from(claim.clone()).unwrap();
		assert_noop!(PoeModule::remove_claim(Origin::signed(2),claim.clone()),Error::<Test>::NotClaimOwner);
	})
}

#[test]
fn claim_not_exist(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		assert_noop!(PoeModule::remove_claim(Origin::signed(1),claim.clone()),
		Error::<Test>::ClaimNotExist);
	})
}
