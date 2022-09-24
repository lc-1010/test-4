#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::{collections::vec_deque::VecDeque, prelude::*, str};

	use frame_system::offchain::{
        Signer,
         CreateSignedTransaction, SendSignedTransaction, 
        
    };
	use sp_io::offchain_index;
    use serde::{Deserialize,Deserializer};
    use sp_runtime::offchain::storage::{StorageValueRef, StorageRetrievalError, MutateStorageError};
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:CreateSignedTransaction<Call<Self>> +frame_system::Config {
        /// The identifier type for an offchain worker.
		type AuthorityId: frame_system::offchain::AppCrypto<Self::Public,Self::Signature> ;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Call: From<Call<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

 
	
	const ONCHAIN_TX_KEY: &[u8] = b"my_pallet::indexing1";
	
	#[derive(Debug, Deserialize, Encode, Decode, Default)]
	struct IndexingData(Vec<u8>, u64);

	 

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(100)]
		pub fn extrinsic(origin:OriginFor<T>,number:u64)-> DispatchResult{
			let who = ensure_signed(origin)?;
			let key = Self::derive_key(frame_system::Module::<T>::block_number());
			let data = IndexingData(b"submit_number".to_vec(), number);

			offchain_index::set(&key, &data.encode());
			Ok(())
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
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet <T> {
		fn offchain_worker(block_number: T::BlockNumber){
			log::info!("开始offchain woker at block number：{:?} ", block_number);

			let payload = vec![1,2,3,4,5,6,7,8];
            _ = Self::send_signed_tx(payload,block_number);

			let key = Self::derive_key(block_number);

			let storage_ref = StorageValueRef::persistent(&key);

			if let Ok(Some(data)) = storage_ref.get::<IndexingData>(){
				log::info!("块  {:?}本地数据 local storage data:  {:?}  {:?} ", 
				block_number,str::from_utf8(&data.0).unwrap_or("error"), data.1);	
			}else{
				log::info!("Error reading from local storage. ");
			}
		}
	}

	impl<T:Config> Pallet<T>{
		
		// 生成索引
		fn derive_key(block_number: T::BlockNumber) ->Vec<u8>{
			block_number.using_encoded(|encode_bn|{
				ONCHAIN_TX_KEY.clone().into_iter()
				.chain(b"/".into_iter())
				.chain(encode_bn)
				.copied()
				.collect::<Vec<u8>>()
			})
		}

		fn send_signed_tx(payload: Vec<u8>,block_number: T::BlockNumber)->Result<(),&'static str>{
            // KEYSTORE 中取
		
            let signer = Signer::<T, T::AuthorityId>::all_accounts();

            if !signer.can_sign(){
                return Err(
                        "No local accounts avilabe. Consider adding one via `author_insertKey` RPC",
                )
            }
			let number: u64 = block_number.try_into().unwrap_or(0);

            let results = signer.send_signed_transaction(|_account|{
                Call::submit_data::<T>{payload: payload.clone() };
				Call::extrinsic{ number: number }
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

	}
}
