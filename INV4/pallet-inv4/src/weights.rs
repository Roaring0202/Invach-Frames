//! Autogenerated weights for pallet_inv4
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Gabriels-MacBook-Pro-3.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
    // ./target/release/invarch-collator
    // benchmark
    // pallet
    // --chain=dev
    // --execution=wasm
    // --wasm-execution=compiled
    // --pallet=pallet_inv4
    // --extrinsic=*
    // --steps
    // 50
    // --repeat
    // 20
    // --heap-pages=4096
    // --log=info
    // --output=../InvArch-Frames/INV4/pallet-inv4/src/weights.rs
    // --template=weights-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_inv4.
pub trait WeightInfo {
	  fn create_core(m: u32, ) -> Weight;
	  fn set_parameters(m: u32, ) -> Weight;
	  fn token_mint() -> Weight;
	  fn token_burn() -> Weight;
	  fn operate_multisig(m: u32, z: u32, ) -> Weight;
	  fn vote_multisig() -> Weight;
	  fn withdraw_vote_multisig() -> Weight;
	  fn cancel_multisig_proposal() -> Weight;
  }

  /// Weights for pallet_inv4 using the Substrate node and recommended hardware.
  pub struct SubstrateWeight<T>(PhantomData<T>);
          impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	            // Storage: INV4 NextCoreId (r:1 w:1)
	            // Storage: CoreAssets Accounts (r:1 w:1)
	            // Storage: CoreAssets TotalIssuance (r:1 w:1)
	            // Storage: System Account (r:2 w:2)
	            // Storage: INV4 CoreByAccount (r:0 w:1)
	            // Storage: INV4 CoreStorage (r:0 w:1)
	            // Storage: INV4 CoreMembers (r:0 w:1)
	            /// The range of component `m` is `[0, 10000]`.
	        fn create_core(m: u32, ) -> Weight {
		      // Minimum execution time: 76_000 nanoseconds.
		      Weight::from_ref_time(77_397_723)
			        // Standard Error: 8
			        .saturating_add(Weight::from_ref_time(788).saturating_mul(m.into()))
			        .saturating_add(T::DbWeight::get().reads(5))
			        .saturating_add(T::DbWeight::get().writes(8))
	        }
	            // Storage: INV4 CoreStorage (r:1 w:1)
	            /// The range of component `m` is `[0, 10000]`.
	        fn set_parameters(m: u32, ) -> Weight {
		      // Minimum execution time: 17_000 nanoseconds.
		      Weight::from_ref_time(17_467_841)
			        // Standard Error: 5
			        .saturating_add(Weight::from_ref_time(784).saturating_mul(m.into()))
			        .saturating_add(T::DbWeight::get().reads(1))
			        .saturating_add(T::DbWeight::get().writes(1))
	        }
	            // Storage: CoreAssets Accounts (r:1 w:1)
	            // Storage: CoreAssets TotalIssuance (r:1 w:1)
	            // Storage: System Account (r:1 w:1)
	            // Storage: INV4 CoreMembers (r:0 w:1)
	        fn token_mint() -> Weight {
		      // Minimum execution time: 37_000 nanoseconds.
		      Weight::from_ref_time(38_000_000)
			        .saturating_add(T::DbWeight::get().reads(3))
			        .saturating_add(T::DbWeight::get().writes(4))
	        }
	            // Storage: CoreAssets Accounts (r:1 w:1)
	            // Storage: CoreAssets TotalIssuance (r:1 w:1)
	            // Storage: System Account (r:1 w:1)
	            // Storage: INV4 CoreMembers (r:0 w:1)
	        fn token_burn() -> Weight {
		      // Minimum execution time: 39_000 nanoseconds.
		      Weight::from_ref_time(40_000_000)
			        .saturating_add(T::DbWeight::get().reads(3))
			        .saturating_add(T::DbWeight::get().writes(4))
	        }
	            // Storage: CoreAssets Accounts (r:1 w:0)
	            // Storage: INV4 CoreStorage (r:1 w:0)
	            // Storage: CoreAssets TotalIssuance (r:1 w:0)
	            // Storage: INV4 Multisig (r:1 w:1)
	            /// The range of component `m` is `[0, 10000]`.
	            /// The range of component `z` is `[0, 4194294]`.
	        fn operate_multisig(_m: u32, z: u32, ) -> Weight {
		      // Minimum execution time: 33_000 nanoseconds.
		      Weight::from_ref_time(13_054_426)
			        // Standard Error: 2
			        .saturating_add(Weight::from_ref_time(2_295).saturating_mul(z.into()))
			        .saturating_add(T::DbWeight::get().reads(4))
			        .saturating_add(T::DbWeight::get().writes(1))
	        }
	            // Storage: INV4 Multisig (r:1 w:1)
	            // Storage: CoreAssets Accounts (r:1 w:0)
	            // Storage: INV4 CoreStorage (r:1 w:0)
	            // Storage: CoreAssets TotalIssuance (r:1 w:0)
	        fn vote_multisig() -> Weight {
		      // Minimum execution time: 31_000 nanoseconds.
		      Weight::from_ref_time(31_000_000)
			        .saturating_add(T::DbWeight::get().reads(4))
			        .saturating_add(T::DbWeight::get().writes(1))
	        }
	            // Storage: INV4 Multisig (r:1 w:1)
	        fn withdraw_vote_multisig() -> Weight {
		      // Minimum execution time: 20_000 nanoseconds.
		      Weight::from_ref_time(21_000_000)
			        .saturating_add(T::DbWeight::get().reads(1))
			        .saturating_add(T::DbWeight::get().writes(1))
	        }
	            // Storage: INV4 Multisig (r:0 w:1)
	        fn cancel_multisig_proposal() -> Weight {
		      // Minimum execution time: 14_000 nanoseconds.
		      Weight::from_ref_time(14_000_000)
			        .saturating_add(T::DbWeight::get().writes(1))
	        }
  }

  // For backwards compatibility and tests
  impl WeightInfo for () {
	        // Storage: INV4 NextCoreId (r:1 w:1)
	        // Storage: CoreAssets Accounts (r:1 w:1)
	        // Storage: CoreAssets TotalIssuance (r:1 w:1)
	        // Storage: System Account (r:2 w:2)
	        // Storage: INV4 CoreByAccount (r:0 w:1)
	        // Storage: INV4 CoreStorage (r:0 w:1)
	        // Storage: INV4 CoreMembers (r:0 w:1)
	        /// The range of component `m` is `[0, 10000]`.
	    fn create_core(m: u32, ) -> Weight {
		  // Minimum execution time: 76_000 nanoseconds.
		  Weight::from_ref_time(77_397_723)
			    // Standard Error: 8
			    .saturating_add(Weight::from_ref_time(788).saturating_mul(m.into()))
			    .saturating_add(RocksDbWeight::get().reads(5))
			    .saturating_add(RocksDbWeight::get().writes(8))
	    }
	        // Storage: INV4 CoreStorage (r:1 w:1)
	        /// The range of component `m` is `[0, 10000]`.
	    fn set_parameters(m: u32, ) -> Weight {
		  // Minimum execution time: 17_000 nanoseconds.
		  Weight::from_ref_time(17_467_841)
			    // Standard Error: 5
			    .saturating_add(Weight::from_ref_time(784).saturating_mul(m.into()))
			    .saturating_add(RocksDbWeight::get().reads(1))
			    .saturating_add(RocksDbWeight::get().writes(1))
	    }
	        // Storage: CoreAssets Accounts (r:1 w:1)
	        // Storage: CoreAssets TotalIssuance (r:1 w:1)
	        // Storage: System Account (r:1 w:1)
	        // Storage: INV4 CoreMembers (r:0 w:1)
	    fn token_mint() -> Weight {
		  // Minimum execution time: 37_000 nanoseconds.
		  Weight::from_ref_time(38_000_000)
			    .saturating_add(RocksDbWeight::get().reads(3))
			    .saturating_add(RocksDbWeight::get().writes(4))
	    }
	        // Storage: CoreAssets Accounts (r:1 w:1)
	        // Storage: CoreAssets TotalIssuance (r:1 w:1)
	        // Storage: System Account (r:1 w:1)
	        // Storage: INV4 CoreMembers (r:0 w:1)
	    fn token_burn() -> Weight {
		  // Minimum execution time: 39_000 nanoseconds.
		  Weight::from_ref_time(40_000_000)
			    .saturating_add(RocksDbWeight::get().reads(3))
			    .saturating_add(RocksDbWeight::get().writes(4))
	    }
	        // Storage: CoreAssets Accounts (r:1 w:0)
	        // Storage: INV4 CoreStorage (r:1 w:0)
	        // Storage: CoreAssets TotalIssuance (r:1 w:0)
	        // Storage: INV4 Multisig (r:1 w:1)
	        /// The range of component `m` is `[0, 10000]`.
	        /// The range of component `z` is `[0, 4194294]`.
	    fn operate_multisig(_m: u32, z: u32, ) -> Weight {
		  // Minimum execution time: 33_000 nanoseconds.
		  Weight::from_ref_time(13_054_426)
			    // Standard Error: 2
			    .saturating_add(Weight::from_ref_time(2_295).saturating_mul(z.into()))
			    .saturating_add(RocksDbWeight::get().reads(4))
			    .saturating_add(RocksDbWeight::get().writes(1))
	    }
	        // Storage: INV4 Multisig (r:1 w:1)
	        // Storage: CoreAssets Accounts (r:1 w:0)
	        // Storage: INV4 CoreStorage (r:1 w:0)
	        // Storage: CoreAssets TotalIssuance (r:1 w:0)
	    fn vote_multisig() -> Weight {
		  // Minimum execution time: 31_000 nanoseconds.
		  Weight::from_ref_time(31_000_000)
			    .saturating_add(RocksDbWeight::get().reads(4))
			    .saturating_add(RocksDbWeight::get().writes(1))
	    }
	        // Storage: INV4 Multisig (r:1 w:1)
	    fn withdraw_vote_multisig() -> Weight {
		  // Minimum execution time: 20_000 nanoseconds.
		  Weight::from_ref_time(21_000_000)
			    .saturating_add(RocksDbWeight::get().reads(1))
			    .saturating_add(RocksDbWeight::get().writes(1))
	    }
	        // Storage: INV4 Multisig (r:0 w:1)
	    fn cancel_multisig_proposal() -> Weight {
		  // Minimum execution time: 14_000 nanoseconds.
		  Weight::from_ref_time(14_000_000)
			    .saturating_add(RocksDbWeight::get().writes(1))
	    }
  }
