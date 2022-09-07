#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
/*！
# kittes 链的功能
 - 领养
 - 繁殖
 - 赠与
 与之前的poe 模块类似，但是扩展内容包括，质押和使用token,使用balance 模块相关功能,同时编写测试代码
*/

#[frame_support::pallet]
pub mod pallet {

	// - import mod
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	//sp_io at the cargo.toml  [dependencies] table
	use sp_io::prelude::*;

	type KittyIndex = u32;

	#[pallet::type_value]
	pub fn GetDefaultValue() -> KittyIndex {
		0_u32
	}

	// - kitty map save
	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub [u8; 16]);

	// - defined config
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// - constant value
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKiityId<T> = StorageValue<_, KittyIndex, ValueQuery, GetDefaultValue>;

	#[pallet::storage]
	#[pallet::getter(fn kittes)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, T::AccountId>;
	// - defined Event
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, KittyIndex, Kitty),
		KittyAdopt(T::AccountId, Vec<u8>),
		KittyBreed(T::AccountId, Vec<u8>),
		KittyTransfor(T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		KittyAlreadExist,
		KittyNotExist,
		NotKittyOwner,
		InvaidKittyId,
		SameKittyId,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_00)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvaidKittyId)?;
			let dna = Self::random_value(&who);
			let kitty = Kitty(dna);

			Self::storage_kitty(kitty_id, kitty, who);
			Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyIndex,
			kitty_id_2: KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			//check kitty id -1
			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvaidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvaidKittyId)?;

			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvaidKittyId)?;

			//breed
			let selecter = Self::random_value(&who);

			let mut data = [0u8; 16];
			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selecter[i] | (kitty_2.0[i] & !selecter[i]));
			}

			let new_kitty = Kitty(data);

			Self::storage_kitty(kitty_id, new_kitty, who);

			Self::deposit_event(Event::KittyCreated(who, kitty_id, new_kitty));
			// check kitty id -2

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// 輔助函數
		// get a random 256
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let palyload = (
				T::Randomness::random_send(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			palyload.using_encoded(blake2_128)
		}

		// get next id
		fn get_next_id() -> Result<KittyIndex, ()> {
			match Self::next_kitty_id() {
				KittyIndex::Max => Err(()),
				val => Ok(val),
			}
		}

		// get kitty via id
		fn get_kitty(kitty_id: KittyIndex) -> Result<Kitty, ()> {
			match Self::kittes(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}

		// new  kitty storage
		fn storage_kitty(kitty_id: KittyIndex, kitty: Kitty, who: T::AccountId) {
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKiityId::<T>::set(kitty_id + 1);
		}
	}
}
