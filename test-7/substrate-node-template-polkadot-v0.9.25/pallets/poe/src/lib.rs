#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet{
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config{
        #[pallet::constant]
        type MaxClinetLenght:Get<u32>;
        type Event:From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    pub type Proofs<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    BoundedVec<u8,T::MaxClinetLenght>,
    (T::AccountId, T::BlockNumber),
    >;

    pub enum Event<T:Confg>{
        ClaimCreated(T::AccountId,Vec<u8>),
        ClaimRevoked(T::AccountId,Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T>{
        ProofAlreadyExist,
        ClaimTooLong,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}
    
}