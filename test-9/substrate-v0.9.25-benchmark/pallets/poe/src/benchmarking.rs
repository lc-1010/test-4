//! Benchmarking setup for pallet-template
//weeightToFee 设置权重
// fee
//
// native wasm-time wasmi
// RocksDb
//
//cargo build --release --features runtime-benchmarks
/* ./target/production/substrate benchmark pallet \
    --chain dev \                  # Configurable Chain Spec
    --execution=wasm \             # Always test with Wasm
    --wasm-execution=compiled \    # Always used `wasm-time`
    --pallet pallet_balances \     # Select the pallet
    --extrinsic transfer \         # Select the extrinsic
    --steps 50 \                   # Number of samples across component ranges
    --repeat 20 \                  # Number of times we repeat a benchmark
    --output <path> \              # Output benchmark results into a folder or file

	./target/production/substrate benchmark pallet \
	--chain dev \
	--execution=wasm \
	--wasm-execution=compiled \
	--pallet pallet_poe --extrinsic "*" \
	--steps 50 \
	--repeat 20 \
	--output ./pallets/poe/src/weights.rs \
	--template .maintain/frame-weight-template.hbs


*/
use crate::*;
use frame_benchmarking::{account,benchmarks,whitelisted_caller};
use frame_system::RawOrigin;

benchmarks!{
	create_claim{
		let d in 0 .. T::MaxClinetLenght::get();
		let claim = vec![0;d as usize];
		let caller: T::AccountId = whitelisted_caller();
	}:_(RawOrigin::Signed(caller),claim)
	// verify{
	//
	// }

	remove_claim{
		let d in 0 .. T::MaxClinetLenght::get();
		let claim = vec![0;d as usize];	
		
		let caller: T::AccountId = whitelisted_caller();
		assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
	}:_(RawOrigin::Signed(caller),claim)

	transfer_claim{
		let d in 0.. T::MaxClinetLenght::get();
		let claim = vec![0;d as usize];
		let caller: T::AccountId = whitelisted_caller();
		assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());

		let dest: T::AccountId= account("dest", 0, 0);

	}:_(RawOrigin::Signed(caller),claim,dest )
	impl_benchmark_test_suite!(PoeModule,crate::mock::new_test_ext(), crate::mock::Test);
}
