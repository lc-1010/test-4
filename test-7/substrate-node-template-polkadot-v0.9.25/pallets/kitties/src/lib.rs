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
	use frame_support::{pallet_prelude::*, Parameter};
	use frame_system::pallet_prelude::*;
	//sp_io at the cargo.toml  [dependencies] table
    use frame_support::traits::Randomness;
    use sp_io::hashing::blake2_128; 
     
    use frame_support::traits::{Currency,ReservableCurrency,ExistenceRequirement, LockableCurrency};
 
	use sp_runtime::traits::AtLeast32BitUnsigned;
    use sp_runtime::traits::Bounded;
    use sp_runtime::traits::One;
    use sp_runtime::traits::CheckedAdd;
    
   

    //type KittyIndex = u32;
	#[pallet::type_value]
	pub fn GetDefaultValue<T:Config>() ->T::KittyIndex{
		0_u32.into()  
	}

	// - kitty map save
	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub [u8; 16]);

	// - defined config
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// - constant value
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;  
        type Currency: ReservableCurrency<Self::AccountId>+ Currency<Self::AccountId> + LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
        type KittyIndex: Parameter
        + Member
        + AtLeast32BitUnsigned
        + Bounded
        + One
        + Default
        +Copy
        +MaybeSerializeDeserialize
        + MaxEncodedLen
       ;
        #[pallet::constant]
        type MinLock: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T:Config> = StorageValue<_, T::KittyIndex, ValueQuery, GetDefaultValue<T>>;

	#[pallet::storage]
	#[pallet::getter(fn kittes)]
	pub type Kitties<T:Config> = StorageMap<_, Blake2_128Concat,T::KittyIndex, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;
	
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance; 
     
    
    // - defined Event
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, T::KittyIndex, Kitty), 
		KittyBreed(T::AccountId, T::KittyIndex),
		KittyTransfor(T::AccountId, T::AccountId,T::KittyIndex),
        SellKitty(T::AccountId,T::AccountId,BalanceOf<T>,T::KittyIndex),
        BuyKitty(T::AccountId,T::AccountId,BalanceOf<T>,T::KittyIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		KittyAlreadExist,
		KittyNotExist,
		NotKittyOwner,
		InvaidKittyId,
		SameKittyId,
        KittyIndexOverFlow,
        MoneyNotEnough, 
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_00)]
		pub fn create(origin: OriginFor<T>,) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvaidKittyId)?;
			let dna = Self::random_value(&who);
			let kitty = Kitty(dna); 
            //锁定钱
            T::Currency::reserve(&who,T::MinLock::get())?;
            Self::storage_kitty(kitty_id, &kitty, &who);
            Self::set_next_id(kitty_id)?; 
			Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
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
                // choose kitty2 and 1 choose kitty1
				data[i] = (kitty_1.0[i] & selecter[i] ) | (kitty_2.0[i] & !selecter[i]);
			}

			let new_kitty = Kitty(data);

			Self::storage_kitty(kitty_id, &new_kitty, &who);
            Self::set_next_id(kitty_id)?; 
			Self::deposit_event(Event::KittyBreed(who, kitty_id));
			// check kitty id -2

			Ok(())
		}

        #[pallet::weight(10_000)]
        pub fn transfor(origin:OriginFor<T>, kitty_id: T::KittyIndex, new_owner:T::AccountId)->DispatchResult{
            let  sender = Self::before_transfor_check_owner(origin,kitty_id)?;
            KittyOwner::<T>::insert(kitty_id,&sender);
            //质押 
            T::Currency::reserve(&new_owner,T::MinLock::get())?;
            let sender_balance = T::Currency::unreserve(&sender,T::MinLock::get());
            
            log::info!("释放质押后balan {:?}",sender_balance);
            Self::deposit_event(Event::KittyTransfor(sender,new_owner,kitty_id));
            Ok(())
        }
         
        #[pallet::weight(10_000)]
        pub fn sell(origin:OriginFor<T>,kitty_id: T::KittyIndex,buyer:T::AccountId,price:BalanceOf<T> )->DispatchResult{
             let who = ensure_signed(origin.clone())?;
            //校验钱包 
            let total = price + T::MinLock::get();
            ensure!(T::Currency::can_slash(&who,total),Error::<T>::MoneyNotEnough);
            //扣押钱包
            T::Currency::transfer(&buyer,&who,price,ExistenceRequirement::KeepAlive)?;
            //转移
            Self::transfor(origin,kitty_id,buyer.clone())?; 
            Self::deposit_event(Event::SellKitty(buyer,who,price,kitty_id)); 
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn buy(origin:OriginFor<T>,kitty_id: T::KittyIndex,owner:T::AccountId,price:BalanceOf<T> )->DispatchResult{
            let who = ensure_signed(origin.clone())?;
           //校验钱包 
           let total = price + T::MinLock::get();
           ensure!(T::Currency::can_slash(&who,total),Error::<T>::MoneyNotEnough);
           //扣押钱包
           T::Currency::transfer(&who,&who,price,ExistenceRequirement::KeepAlive)?;
           //转移
           Self::transfor(origin,kitty_id,who.clone())?; 
           Self::deposit_event(Event::SellKitty(who,owner,price,kitty_id)); 
           Ok(())
       }
         
	}

	impl<T: Config> Pallet<T> {
		// 輔助函數 
		// get a random 256
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let palyload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			palyload.using_encoded(blake2_128)
		}

		// get next id
		fn get_next_id() -> Result<T::KittyIndex, Error<T>> {
			match Self::next_kitty_id() {
                val => {
                    ensure!(val!= T::KittyIndex::max_value(), Error::<T>::KittyIndexOverFlow);
                    Ok(val)
                }, 
			}
		}

		// get kitty via id
		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()> {
			match Self::kittes(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}

		// new  kitty storage
		fn storage_kitty(kitty_id: T::KittyIndex, kitty: &Kitty, who: & T::AccountId) {
			Kitties::<T>::insert(kitty_id, kitty);
			KittyOwner::<T>::insert(kitty_id, who); 
		}

        fn set_next_id(kitty_id: T::KittyIndex)->Result<(),DispatchError >{

            let id = T::KittyIndex::one();
            let next_id = kitty_id.checked_add(&id).ok_or(
                Error::<T>::KittyIndexOverFlow
            )?;
            NextKittyId::<T>::set(next_id);
            Ok(())
        }

        // 交易 转移检查
        fn before_transfor_check_owner(origin:OriginFor<T>, kitty_id: T::KittyIndex)->Result<T::AccountId,DispatchError>{
            let who = ensure_signed(origin)?;
            Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvaidKittyId)?;
            ensure!(Self::kitty_owner(kitty_id)==Some(who.clone()),Error::<T>::NotKittyOwner);
            Ok(who)
        }

        
         
	}
}
