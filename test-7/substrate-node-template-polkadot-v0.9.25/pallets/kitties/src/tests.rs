use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		//所有者
		assert_eq!(KittyModule::kitty_owner(0), Some(1));
		//扣押
		assert_eq!(Balances::reserved_balance(1), MinLock::get().try_into().unwrap());
	})
}
#[test]
fn create_kitty_error_moneynotenough() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittyModule::create(Origin::signed(6)), Error::<Test>::MoneyNotEnough);
	})
}

#[test]
fn create_with_error_kittyindexoverflow() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::set_next_id(<Test as Config>::KittyIndex::max_value() - 1));
		assert_noop!(KittyModule::create(Origin::signed(1)), Error::<Test>::KittyIndexOverFlow);
	})
}

#[test]
fn bread_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::breed(Origin::signed(1), 0, 1));
	})
}

#[test]
fn bread_kitty_error_invaidkittyid() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_noop!(KittyModule::breed(Origin::signed(1), 3, 1), Error::<Test>::InvaidKittyId);
	})
}
#[test]
fn bread_kitty_error_samekittyid() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_noop!(KittyModule::breed(Origin::signed(1), 1, 1), Error::<Test>::SameKittyId);
	})
}
/*
没有 new_test_ext().execute_with 会报错
thread 'tests::transfer' panicked at '`get_version_1` called outside of an Externalities-provided environment.',
	//.cargo/git/checkouts/substrate-7e08433d4c370a21/3348e14/primitives/io/src/lib.rs:127:5
*/
#[test]
fn transfer() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::transfer(Origin::signed(1), 0, 3));
		//质押 3 MinLock recev
		assert_eq!(KittyModule::kitty_owner(0), Some(3));
		assert_eq!(Balances::reserved_balance(3), MinLock::get().try_into().unwrap());
		assert_eq!(Balances::free_balance(3), 9996);
		assert_eq!(Balances::free_balance(1), 100_000_000_000); //释放质押
	})
}

#[test]
fn transfer_error_notkittywwner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::create(Origin::signed(2)));
		assert_noop!(KittyModule::transfer(Origin::signed(1), 1, 3), Error::<Test>::NotKittyOwner);
	})
}
#[test]
fn transfer_err_invaidkittyid() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_noop!(KittyModule::transfer(Origin::signed(1), 1, 3), Error::<Test>::InvaidKittyId);
	})
}

#[test]
fn transfer_err_sameaccount() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_noop!(KittyModule::transfer(Origin::signed(1), 0, 1), Error::<Test>::SameAccount);
	})
}

#[test]
fn transfer_err_moneynotenough() {
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(Origin::root(), 6, 1, 0));
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_noop!(KittyModule::transfer(Origin::signed(1), 0, 6), Error::<Test>::MoneyNotEnough);
	})
}
/*
https://stackoverflow.com/questions/66984081/assert-noop-doesnt-pass-the-test
在lib 中逻辑错误 造成了storage 改变 先检查后修改，没有回滚
assert_noop!(KittyModule::transfer(Origin::signed(1), 0, 6),  Error::<Test>::MoneyNotEnough);

thread 'tests::transfer_err_moneynotenough' panicked at 'assertion failed: `(left == right)`
  left: `[47, 83, 223, 53, 103, 114, 85, 163, 91, 23, 202, 147, 98, 107, 163, 204, 170, 70, 125, 27, 187, 244, 79, 82, 139, 86, 208, 21, 161, 84, 0, 53]`,
 right: `[225, 195, 5, 254, 140, 20, 141, 108, 101, 244, 119, 95, 37, 206, 167, 95, 237, 21, 227, 95, 146, 222, 119, 51, 173, 39, 180, 104, 112, 43, 157, 93]`', pallets/kitties/src/tests.rs:113:9
stack backtrace:
   0: rust_begin_unwind
*/

#[test]
fn sell() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::sell(Origin::signed(1), 0, 2, 100));

		assert_eq!(KittyModule::kitty_owner(0), Some(2));
		assert_eq!(Balances::reserved_balance(2), MinLock::get().try_into().unwrap());
		assert_eq!(Balances::free_balance(2), 100_000_000_000 - 3 - 100);
		assert_eq!(Balances::free_balance(1), 100_000_000_000 + 100); //释放质押
	})
}

#[test]
fn sell_error_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_noop!(KittyModule::sell(Origin::signed(2), 0, 2, 100), Error::<Test>::NotKittyOwner);
	})
}

#[test]
fn sell_error_money() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_noop!(
			KittyModule::sell(Origin::signed(1), 0, 3, 11111100),
			Error::<Test>::MoneyNotEnough
		);
	})
}

#[test]
fn buy() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_ok!(KittyModule::buy(Origin::signed(2), 0, 1, 10));

		assert_eq!(KittyModule::kitty_owner(0), Some(2));
		//assert_eq!(Balances::reserved_balance(2), MinLock::get().try_into().unwrap());
		assert_eq!(Balances::free_balance(2), 100_000_000_000 - 3 - 10);
		assert_eq!(Balances::free_balance(1), 100_000_000_000 + 10); //释放质押
	})
}

#[test]
fn buy_error_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(1)));
		assert_noop!(KittyModule::buy(Origin::signed(2), 0, 2, 100), Error::<Test>::NotKittyOwner);
	})
}

#[test]
fn buy_error_money() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittyModule::create(Origin::signed(3)));
		assert_noop!(
			KittyModule::buy(Origin::signed(6), 0, 3, 11111100),
			Error::<Test>::MoneyNotEnough
		);
	})
}
