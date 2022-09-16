use crate as pallet_kitties;
use frame_support::{traits::{ConstU16, ConstU32, ConstU64, ConstU128}, parameter_types};
use sp_core::H256;
use frame_system as system;// 别名
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		//系统
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		//runtime 中 kitty 模块
		KittyModule: pallet_kitties::{Pallet, Call, Storage, Event<T>},
		//账户模块
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		//随机dna
		Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
	}
);

//system 配置项目
impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// 账户模块
impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<500>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

// 重要实现
impl pallet_randomness_collective_flip::Config for Test {}

parameter_types! {
	//pub const Reserved: u128 = 10_000;
	// 创建一个kitty时要锁定的最小 金额 reserved 值
	// 转移和卖出后释放
	pub const MinLock: u32 = 3;
}

impl pallet_kitties::Config for Test {
	type Event = Event;
	type Randomness = Randomness;
	type KittyIndex = u32;
	// 账户
	type Currency = Balances;
	// 配置
	type MinLock=MinLock;
}

// 宏
// #[macro_export]
// macro_rules! assert_has_event
// {
// 	($x:expr) =>
// 	{
// 		System::assert_has_event(TestEvent::Kitties($x))
// 	};
// }

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
		 

	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test>
	{
		balances: vec![(1, 100_000_000_000), (2, 100_000_000_000), (3, 9999), (4, 20_000),],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	 

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
