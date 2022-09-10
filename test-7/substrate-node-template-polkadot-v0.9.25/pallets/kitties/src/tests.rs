use super::*;
use crate::{mock::*, Error};
use super::{Event as KittyEvent};
use frame_support::{assert_noop, assert_ok};
use pallet_balances::Error as BalancesError;
/*
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
*/
#[test]
fn create_kitty(){
	new_test_ext().execute_with(||{
		 
	})
}