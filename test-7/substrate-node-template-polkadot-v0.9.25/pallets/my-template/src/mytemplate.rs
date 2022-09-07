pub use pallet::*;
// 引入pallet
#[frame_supprt::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::config]
  pub trait Config: frame_system::Config{}

  #[pallet::pallet]
  #[pallet::generate_deposit{pub(super) fn deposit_event)}]
  pub enum Event<T:Config>{

  }


  #[pallet::error]
  pub enum Error<T>{

  }

  #[pallet::call]
  impl <T:Config> Pallet<T>{

  }

  impl <T:Config>Pallet<T> {

  }
}
