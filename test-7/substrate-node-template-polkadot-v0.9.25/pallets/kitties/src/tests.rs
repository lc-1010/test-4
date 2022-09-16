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
fn create_kitty(){
	new_test_ext().execute_with(||{
		 
		 assert_ok!(KittyModule::create(Origin::signed(1)));
		 //所有者
		 assert_eq!(KittyModule::kitty_owner(0),Some(1));
		 //扣押
		 assert_eq!(  Balances::reserved_balance(1), MinLock::get().try_into().unwrap());

	})
}
#[test]
fn create_kitty_error_moneynotenough(){
	new_test_ext().execute_with(||{ 
		 assert_noop!(KittyModule::create(Origin::signed(6)),Error::<Test>::MoneyNotEnough);
	})
}

#[test]
fn create_with_error_kittyindexoverflow(){
	new_test_ext().execute_with(||{
		assert_ok!(KittyModule::set_next_id( <Test as Config>::KittyIndex::max_value()-1)); 
		assert_noop!(KittyModule::create(Origin::signed(1)), Error::<Test>::KittyIndexOverFlow);
	})
}

#[test]
fn bread_kitty(){
	new_test_ext().execute_with(||{
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::create(Origin::signed(1)));	
		assert_ok!(KittyModule::breed(Origin::signed(1), 0, 1));
	})
}

#[test]
fn bread_kitty_error_invaidkittyid(){
	new_test_ext().execute_with(||{
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::create(Origin::signed(1)));	
		assert_noop!(KittyModule::breed(Origin::signed(1), 3, 1),Error::<Test>::InvaidKittyId);
	})
}
#[test]
fn bread_kitty_error_samekittyid(){
	new_test_ext().execute_with(||{
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::create(Origin::signed(1)));	
		assert_noop!(KittyModule::breed(Origin::signed(1), 1, 1),Error::<Test>::SameKittyId);
	})
}
/* 
没有 new_test_ext().execute_with 会报错
thread 'tests::transfer' panicked at '`get_version_1` called outside of an Externalities-provided environment.',  
	//.cargo/git/checkouts/substrate-7e08433d4c370a21/3348e14/primitives/io/src/lib.rs:127:5
*/
#[test]
fn transfer(){
	new_test_ext().execute_with(||{ 
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::transfer(Origin::signed(1), 0, 3));
		//质押 3 MinLock recev
		assert_eq!(KittyModule::kitty_owner(0),Some(3)); 
		assert_eq!(Balances::reserved_balance(3), MinLock::get().try_into().unwrap());
		assert_eq!(Balances::free_balance(3),9996);
		assert_eq!(Balances::free_balance(1),100_000_000_000);//释放质押
	})
}


#[test]
fn sell(){
	new_test_ext().execute_with(||{ 
	})
}

#[test]
fn buy(){
	new_test_ext().execute_with(||{ 
	})
}