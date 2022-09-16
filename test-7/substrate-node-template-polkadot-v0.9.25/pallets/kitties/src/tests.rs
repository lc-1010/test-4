use super::*;
use crate::{mock::*, Error};
use frame_support::{ assert_noop, assert_ok};

/**
 		KittyAlreadExist,
		KittyNotExist,
		NotKittyOwner,
		InvaidKittyId,
		SameKittyId,
        KittyIndexOverFlow,
        MoneyNotEnough,
 */
#[test]
fn create_kitty_succes(){
	new_test_ext().execute_with(||{
		 
		 assert_ok!(KittyModule::create(Origin::signed(1)));
		 //所有者
		 assert_eq!(KittyModule::kitty_owner(0),Some(1));
		 //扣押
		 assert_eq!(  Balances::reserved_balance(1), MinLock::get().try_into().unwrap());

	})
}
#[test]
fn create_kitty_error(){
	new_test_ext().execute_with(||{
		// let _ = Balances::set_balance(Origin::root(), 8, 1,1);
		 assert_noop!(KittyModule::create(Origin::signed(6)),Error::<Test>::MoneyNotEnough);
	})
}

#[test]
fn bread_kitty(){
	new_test_ext().execute_with(||{

	})
}
