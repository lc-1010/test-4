#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//提交签证===》
use sp_core::crypto::KeyTypeId;
// 4个字符
pub const KYE_TYPE: KeyTypeId = KeyTypeId(*b"ocwd");

pub mod crypto {
    use super::KYE_TYPE; 
	use sp_core::sr25519::Signature as Sr25519Signature;

    use sp_runtime::{
        app_crypto::{ app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };
    app_crypto!(sr25519, KYE_TYPE);
    pub struct OcwAuthId;

    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for OcwAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    impl frame_system::offchain::AppCrypto< <Sr25519Signature as Verify>::Signer,Sr25519Signature>
    for OcwAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}
//《===提交签证


// use sp_runtime::{
//     transaction_validity::{InvalidTransaction,TransactionValidity, ValidTransaction},
//      {  http,Duration, },
// }; 

 
#[frame_support::pallet]
pub mod pallet {
    //use supper::*; //引入外部
    use frame_system::offchain::{
        Signer,
         CreateSignedTransaction, SendSignedTransaction, 
        
    };
   
    use frame_support::pallet_prelude::*;
	 
    use frame_system::{pallet_prelude::*, ensure_signed};
    //import
	use frame_support::pallet_prelude::Hooks;
    use serde::{Deserialize,Deserializer};
    use sp_runtime::offchain::{http, Duration};
    use sp_runtime::traits::Zero;
    use sp_runtime::offchain::storage::{StorageValueRef, StorageRetrievalError, MutateStorageError};
       
    use sp_std::vec;
    use sp_std::vec::Vec;
    // 编码 

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

    /*  注释掉后会使用原始 【Debug】打印 原始 vec 数据
    { login: [115, 117, 98, 115, 116, 114, 97, 116, 101, 45
        , 100, 101, 118, 101, 108, 111, 112, 101, 114, 45, 104, 117, 98], 
        blog: [104, 116, 116, 112, 115, 58, 47,
         47, 119, 119, 119, 46, 115, 117, 98, 115, 116, 114,
          97, 116, 101, 46, 105, 111, 47], public_repos: 37 }
    */
    use core::{convert::TryInto,fmt};
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
    //------crypto



	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config:CreateSignedTransaction<Call<Self>> +frame_system::Config {
        /// The identifier type for an offchain worker.
		type AuthorityId: frame_system::offchain::AppCrypto<Self::Public,Self::Signature> ;

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The overarching dispatch call type.
		type Call: From<Call<Self>>;
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
        // call Dispatch 过来调用
        #[pallet::weight(0)]
        pub fn submit_data(origin: OriginFor<T>,payload:Vec<u8>)->DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;
            log::info!(" 🥰====>in submit_data call {:?}", payload);
            Ok(().into())
        }
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!(
				"♦️ hello world from offchain workers! this is 我的第:{:?} 个块 ♦️",
				block_number
			);

			let timeout = sp_io::offchain::timestamp()
                    .add(sp_runtime::offchain::Duration::from_millis(3));
			sp_io::offchain::sleep_until(timeout);

            //read and write
            Self::reads_writes(block_number);
            
            // read github

            if let Ok(info) = Self::read_github(){
                log::info!("Github Info :{:?}", info);
            }else{
                log::info!("Error while fetch github info!");
            }

            let payload = vec![1,2,3,4,5,6,7,8];
            _ = Self::send_signed_tx(payload);

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

    // 辅助函数
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
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
            let request = http::Request::get("https://api.github.com/orgs/substrate-developer-hub");
            let pending = request
            .add_header("User-Agent", "Substrate-Offchain-Worker")
            .deadline(deadline)
            .send().map_err(|_|http::Error::IoError)?;

            let response = pending
                    .try_wait(deadline)
                    .map_err(|_|http::Error::DeadlineReached )??;

            if response.code != 200 {
                log::warn!("Unexpected status code:{}", response.code);
                return Err(http::Error::Unknown)
            }
            let body = response.body().collect::<Vec<u8>>();
            let body_str = sp_std::str::from_utf8(&body).map_err(|_|{
                log::warn!("No utf8 body");
                http::Error::Unknown
            })?;

            let gh_info:GithubInfo = serde_json::from_str(body_str)
            .map_err(|_|http::Error::Unknown)?;

            Ok(gh_info)
        }

        fn send_signed_tx(payload: Vec<u8>)->Result<(),&'static str>{
            // KEYSTORE 中取
            let signer = Signer::<T, T::AuthorityId>::all_accounts();

            if !signer.can_sign(){
                return Err(
                        "No local accounts avilabe. Consider adding one via `author_insertKey` RPC",
                )
            }

            let results = signer.send_signed_transaction(|_account|{
                Call::submit_data {payload: payload.clone() }
                // 内部 枚举 方法名-字段 data 映射 match
            });
            // vec 多个 
            for (acc,res) in &results {
                match res{
                    Ok(()) =>log::info!("😇 成功提交[{:?}] submitted data: {:?},", acc.id, payload),
                    Err(e) =>log::error!("�� 失败啦 [{:?}] Failed to submit transaction: {:?}",acc.id, e),
                }
            }
            Ok(())
        }

        // fn transaction(bolck_number: T::BlockNumber){
        //     let val:u64 =43;
        //     let call = Call::submit_unsigned_transaction{ n:value};

        //     _= SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
        //         .map_err(|_|{
        //             log::error!("Failed in offchain_unsigned_tx");
        //         });
        //     log::info!("Leave offchain worers!:{:?}", bolck_number);
        // }
    }

    // #[pallet::validate_unsigned]
    // impl<T:Config>ValidateUnsigned for Pallet<T>{
    //     type Call = Call<T>;

    //     fn validate_unsigned(_soure: TransactionSource, call:&Self::Call)-> TransactionValidity{
    //         if let Call::submit_data_unsigned{ n:_} =call {
    //             ValidTransaction::with_tag_perfix("MyOffchainWorker")
    //                 .priority(10000)
    //                 .and_provides(1)
    //                 .longevity(3)
    //                 .proagate(true)
    //                 .build()
    //         }else{
    //             InvalidTransaction::Call.into()
    //         }
    //     }
    // }
}
