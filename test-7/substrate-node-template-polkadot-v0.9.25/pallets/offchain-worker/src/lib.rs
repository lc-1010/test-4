#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {


use frame_support::{pallet_prelude::*, Deserialize};
	use frame_system::pallet_prelude::*;
    //import
	use frame_support::pallet_prelude::Hooks;
    use sp_runtime::traits::Zero;
    use sp_runtime::offchain::storage::{StorageValueRef, StorageRetrievalError, MutateStorageError};
    use sp_runtime::{
        offchain::{
            http,Duration,
        },
    };
    use frame_support::inherent::Vec;
    use serde::{ Deserializer};


    #[derive(Deserialize,Encode,Decode)]
    struct GithubInfo{
        #[serde(deserialize_with = "de_string_to_bytes")]
        login:Vec<u8>,
        #[serde(deserialize_with = "de_string_to_bytes")]
        blog:Vec<u8>,
        public_repos: u32,
    }

    pub fn de_string_to_bytes<'de,D>(de:D)->Result<Vec<u8>,D::Error>
    where
    D: Deserializer<'de>,
    {
        let s:&str = Deserialize::deserialize(de)?;
        Ok(s.as_bytes().to_vec())

    }

    use core::{conver::IryInto,fmt};
    impl fmt::Debug for GithubInfo{
        fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{{ login:{}, blog:{}, public_repos:{} }}",
                sp_std::str::from_utf8(&self.login).map_err(|_| fmt::Error)?,
                sp_std::str::from_utf8(&self.blog).map_err(|_|fmt::Error)?,
                &self.public_repos
            )
        }
    }


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super)fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000+T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Something<T>>::put(something);
			Self::deposit_event(Event::SomethingStored(something, who));
			Ok(())
		}

		#[pallet::weight(10_000 +T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			match <Something<T>>::get() {
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!(
				"♦️ hello world from offchain workers! this is 我的第:{:?} 个块 ♦️",
				block_number
			);

			let timeout =
				sp_io::offchain::timestamp().add(sp_runtime::offchain::Duration::from_millis(1000));
			sp_io::offchain::sleep_until(timeout);

            //read and write
            Self::reads_writes(block_number);
            // read github
            if let Ok(info) = Self::read_github(){
                log::info!("Github Info :{:?}", info);
            }else{
                log::info!("Error while fetch github info!");
            }
			log::info!("Leave from offchain workers!: {:?} ♦️ ", block_number);
		}
		// on_initailz
		fn on_initialize(n: T::BlockNumber) -> Weight {
			log::info!("♦️ in on_initialize! 这是第:{:?}", n);
			Weight::zero()
		}

		fn on_finalize(n: T::BlockNumber) {
			log::info!("♦️ in on_finalize!,这是第:{:?}", n);
		}

		fn on_idle(n: T::BlockNumber, _remaining_weight: Weight) -> Weight {
			log::info!("♦️ in on_idle!,这是第:{:?}", n);
			0
		}
	}

    impl<T:Config> Pallet<T>{
        #[deny(clippy::clone_double_ref)]
        fn derive_key(block_number: T::BlockNumber)->Vec<u8>{
            block_number.using_encoded(|encode_bn|{
                b"node-template::storage::"
                .iter()
                .chain(encode_bn)
                .copied()
                .collect::<Vec<u8>>()
            })
        }

        fn reads_writes(block_number: T::BlockNumber){
            if block_number % 2u32.into() != Zero::zero(){
                //odd
                let key = Self::derive_key(block_number);
                // 模板方法
                let val_ref = StorageValueRef::persistent(&key);

                let random_slice = sp_io::offchain::random_seed();
                let timestamp_u64 = sp_io::offchain::timestamp().unix_millis();

                let value = (random_slice,timestamp_u64);

                log::info!("🥳==>in odd block ,value to write:{:?} ",value);
                //val_ref.set(&value);
                // 并发不推荐用set
                struct StateError;//推导用
                let res = val_ref.mutate(|val:Result<Option<([u8;32],u64)>,StorageRetrievalError>|->Result<_,StateError> {

                    match val{
                        Ok(Some(_))=>Ok(value),
                        _ => Ok(value),
                    }
                });

                match res {
                    Ok(value) =>{
                        log::info!("🥳===>in odd block, mutate successfully: {:?},", value);
                    },
                    Err(MutateStorageError::ValueFunctionFailed(_))=>(),
                    Err(MutateStorageError::ConcurrentModification(_))=>(),
                }
            }else{
                //even
                let key = Self::derive_key(block_number-1u32.into());
                let mut val_ref = StorageValueRef::persistent(&key);

                if let Ok(Some(value)) = val_ref.get::<([u8;32],u64)>(){
                    log::info!("🥳==>in even block, value read:{:?}", value);
                    val_ref.clear();
                }

            }
        }

        fn read_github()->Result<GithubInfo,http::Error>{
            log::info!("====>start read github");
            let dealline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
            let request = http::Request::get("https://api.github.com/orgs/substrate-developer-hub");
            let pending = request
            .add_header("User-Agent", "Substrate-Offchain-Worker")
            .deadline(dealline)
            .send().map_err(|_|http::Error::IoError)?;

            let response = pending.try_wait(deadline).map_err(|_|{
                http:Error::DeadlineReached
            })??;

            if response.code != 200 {
                log::warn!("Unexpected status code:{}", response.code);
                return Err(http::Error::Unknow);
            }
            let body = response.body().collect::<Vec<u8>>();
            let body_str = sp_std::str::from_utf8(&body).map_err(|_|{
                log::warn!("No utf8 body");
                http::Error::Unkow
            })?;

            let gh_info:GithubInfo = serde_json::from_str(body_str).map_err(|_|http::Error::Unknow)?;

            Ok(gh_info)
        }
    }
}
