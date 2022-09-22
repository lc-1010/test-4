#![cfg_attr(not(feature="std"),no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Zero;
    use frame_support::pallet_prelude::Hooks;

    
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
         
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
    
    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_,u32>;


    #[pallet::event]
    #[pallet::generate_deposit(pub (super)fn deposit_event)]
    pub enum Event<T:Config>{
        SomethingStored(u32,T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T>{
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl <T:Config> Pallet<T>{
        #[pallet::weight(100_000+T::DbWeight::get().writes(1))]
        pub fn do_something(origin:OriginFor<T>,something:u32)->DispatchResult {
            let who = ensure_signed(origin)?;
            <Something<T>>::put(something);
            Self::deposit_event(Event::SomethingStored(something,who));
            Ok(())
        }

        #[pallet::weight(10_000 +T::DbWeight::get().reads_writes(1,1))]
        pub fn cause_error(origin: OriginFor<T>)->DispatchResult {
            let _who = ensure_signed(origin)?;
            match <Something<T>>::get(){
                None => return Err(Error::<T>::NoneValue.into()),
                Some(old)=>{
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    <Something<T>>::put(new);
                    Ok(())
                    
                }
            }
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
           fn offchain_worker(block_number: T::BlockNumber){
            log::info!("♦️ hello world from offchain workers! this is 我的第:{:?} 个块 ♦️", block_number);
           }
            // on_initailz 
            fn on_initialize(n: T::BlockNumber) -> Weight {
                log::info!("♦️ in on_initialize! 这是第:{:?}",n);
                Weight::zero()
            }
    
            fn on_finalize(n: T::BlockNumber) {
                log::info!("♦️ in on_finalize!,这是第:{:?}",n);
            }

            fn on_idle (n: T::BlockNumber, _remaining_weight: Weight) -> Weight {
                log::info!("♦️ in on_idle!,这是第:{:?}",n);
                0
            }
        }
}
