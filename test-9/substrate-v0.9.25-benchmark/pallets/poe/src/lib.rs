#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
#[cfg(feature="runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use frame_support::pallet_prelude::*;//pub 公开给benchmark
	pub use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;
	use super::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxClinetLenght: Get<u32>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		//定义接口传递权重值
		type WeightInfo:WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClinetLenght>,
		(T::AccountId, T::BlockNumber),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//#[pallet::weight(0)] runtime 中定义
		#[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
		/**
		claim 存证 origin 发送方
		*/
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClinetLenght>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			ensure!(
				!Proofs::<T>::contains_key(bounded_claim.clone()),
				Error::<T>::ProofAlreadyExist
			);
			let currrent_block = <frame_system::Pallet<T>>::block_number();
			Proofs::<T>::insert(&bounded_claim, (sender.clone(), currrent_block));
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
		}

		//#[pallet::weight(0)]
		#[pallet::weight(T::WeightInfo::remove_claim(claim.len() as u32))]
		pub fn remove_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let bounded_claim = BoundedVec::<u8, T::MaxClinetLenght>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			//ensure!(Proofs::<T>::contains_key(bounded_claim.clone()),Error::<T>::ClaimNotExist);

			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(sender == owner, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&bounded_claim);
			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			Ok(().into())
		}

		//#[pallet::weight(0)]
		#[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]	
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
			dest: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClinetLenght>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			let (owner, _block_number) =
				Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::insert(&bounded_claim, (dest, frame_system::Pallet::<T>::block_number()));
			Ok(().into())
		}
	}
}
