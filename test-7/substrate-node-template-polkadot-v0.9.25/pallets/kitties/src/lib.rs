#![cfg_attr(not(feature="std"),no_std)]
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
    pub fn GetDefaultValue()->KittyIndex {
        0_u32
    }

    // - kitty map 
    #[derive(Encode,Decode,Clone,PartialEq,Eq,Debug,TypeInfo,MaxEncodedLen)]
    pub struct Kitty(pub [u8;16]);


    // - defined config 
    #[pallet::config]
    pub trait Config:frame_system::Config{
        // - constant value
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
   
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    pub type NextKiityId<T> = StorageValue<
    _,
    KittyIndex,
    ValueQuery,
    GetDefaultValue,
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn kittes)]
    pub type kittes<T> = StorageMap<
    _,
    Blake2_128Concat,
    KittyIndex,
    Kitty
    >;

    

    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T:Config> = StorageMap<
    _,
    Blake2_128Concat, 
    KittyIndex,
    T::AccountId,
    >;
    // - defined Event 
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>{
        KittyAdopt(T::AccountId,Vec<u8>),
        KittyBreed(T::AccountId,Vec<u8>),
        KittyTransfor(T::AccountId,Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T>{
        KittyAlreadExist,
        KittyNotExist,
        NotKittyOwner,
    }

    #[pallet::hooks]
    impl <T:Config>Hooks<BlockNumberFor<T>> for Pallet<T>{}

    #[pallet::call]
    impl <T:Config>Pallet<T>{
        // pub fn adopt_kitty(){}
        // pub fn breed_kitty(){}
        // pub fn transfer_kitty(){}
    }
}
