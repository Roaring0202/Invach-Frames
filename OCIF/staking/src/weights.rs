
//! Autogenerated weights for `pallet_ocif_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Gabriels-MacBook-Pro-3.local`, CPU: `<UNKNOWN>`
//! EXECUTION: `Some(Wasm)`, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
    // ./target/release/invarch-collator
    // benchmark
    // pallet
    // --chain=dev
    // --execution=wasm
    // --wasm-execution=compiled
    // --pallet=pallet_ocif_staking
    // --extrinsic=*
    // --steps
    // 50
    // --repeat
    // 20
    // --output=../InvArch-Frames/OCIF/staking/src/weights.rs
    // --template=weights-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_ocif_staking`.
pub trait WeightInfo {
	  fn register_core(n: u32, d: u32, i: u32, ) -> Weight;
	  fn change_core_metadata(n: u32, d: u32, i: u32, ) -> Weight;
	  fn unregister_core() -> Weight;
	  fn stake() -> Weight;
	  fn unstake() -> Weight;
	  fn withdraw_unstaked() -> Weight;
	  fn staker_claim_rewards() -> Weight;
	  fn core_claim_rewards() -> Weight;
	  fn halt_unhalt_pallet() -> Weight;
  }

  /// Weights for `pallet_ocif_staking` using the Substrate node and recommended hardware.
  pub struct SubstrateWeight<T>(PhantomData<T>);
          impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	            /// Storage: OcifStaking Halted (r:1 w:0)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	            /// Storage: OcifStaking RegisteredCore (r:1 w:1)
	            /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	            /// Storage: System Account (r:1 w:1)
	            /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	            /// The range of component `n` is `[0, 20]`.
	            /// The range of component `d` is `[0, 300]`.
	            /// The range of component `i` is `[0, 100]`.
	        fn register_core(_n: u32, _d: u32, i: u32, ) -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `144`
		      //  Estimated: `3942`
		      // Minimum execution time: 28_000_000 picoseconds.
		      Weight::from_parts(29_525_844, 3942)
			        // Standard Error: 641
			        .saturating_add(Weight::from_parts(2_603, 0).saturating_mul(i.into()))
			        .saturating_add(T::DbWeight::get().reads(3_u64))
			        .saturating_add(T::DbWeight::get().writes(2_u64))
	        }
	            /// Storage: OcifStaking Halted (r:1 w:0)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	            /// Storage: OcifStaking RegisteredCore (r:1 w:1)
	            /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	            /// The range of component `n` is `[0, 20]`.
	            /// The range of component `d` is `[0, 300]`.
	            /// The range of component `i` is `[0, 100]`.
	        fn change_core_metadata(n: u32, d: u32, i: u32, ) -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `86`
		      //  Estimated: `3942`
		      // Minimum execution time: 13_000_000 picoseconds.
		      Weight::from_parts(13_415_126, 3942)
			        // Standard Error: 2_128
			        .saturating_add(Weight::from_parts(6_775, 0).saturating_mul(n.into()))
			        // Standard Error: 145
			        .saturating_add(Weight::from_parts(1_501, 0).saturating_mul(d.into()))
			        // Standard Error: 436
			        .saturating_add(Weight::from_parts(3_700, 0).saturating_mul(i.into()))
			        .saturating_add(T::DbWeight::get().reads(2_u64))
			        .saturating_add(T::DbWeight::get().writes(1_u64))
	        }
	            /// Storage: OcifStaking Halted (r:1 w:0)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	            /// Storage: OcifStaking RegisteredCore (r:1 w:1)
	            /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CurrentEra (r:1 w:0)
	            /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralStakerInfo (r:1 w:0)
	            /// Proof: OcifStaking GeneralStakerInfo (max_values: None, max_size: Some(269), added: 2744, mode: MaxEncodedLen)
	            /// Storage: System Account (r:1 w:1)
	            /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	        fn unregister_core() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `268`
		      //  Estimated: `3942`
		      // Minimum execution time: 34_000_000 picoseconds.
		      Weight::from_parts(34_000_000, 3942)
			        .saturating_add(T::DbWeight::get().reads(5_u64))
			        .saturating_add(T::DbWeight::get().writes(2_u64))
	        }
	            /// Storage: OcifStaking Halted (r:1 w:0)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	            /// Storage: OcifStaking RegisteredCore (r:1 w:0)
	            /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	            /// Storage: OcifStaking Ledger (r:1 w:1)
	            /// Proof: OcifStaking Ledger (max_values: None, max_size: Some(265), added: 2740, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CurrentEra (r:1 w:0)
	            /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CoreEraStake (r:1 w:1)
	            /// Proof: OcifStaking CoreEraStake (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralStakerInfo (r:1 w:1)
	            /// Proof: OcifStaking GeneralStakerInfo (max_values: None, max_size: Some(269), added: 2744, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralEraInfo (r:1 w:1)
	            /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	            /// Storage: Balances Locks (r:1 w:1)
	            /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	            /// Storage: Balances Freezes (r:1 w:0)
	            /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	        fn stake() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `86`
		      //  Estimated: `4764`
		      // Minimum execution time: 46_000_000 picoseconds.
		      Weight::from_parts(47_000_000, 4764)
			        .saturating_add(T::DbWeight::get().reads(9_u64))
			        .saturating_add(T::DbWeight::get().writes(5_u64))
	        }
	            /// Storage: OcifStaking Halted (r:1 w:0)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	            /// Storage: OcifStaking RegisteredCore (r:1 w:0)
	            /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CurrentEra (r:1 w:0)
	            /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralStakerInfo (r:1 w:1)
	            /// Proof: OcifStaking GeneralStakerInfo (max_values: None, max_size: Some(269), added: 2744, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CoreEraStake (r:1 w:1)
	            /// Proof: OcifStaking CoreEraStake (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	            /// Storage: OcifStaking Ledger (r:1 w:1)
	            /// Proof: OcifStaking Ledger (max_values: None, max_size: Some(265), added: 2740, mode: MaxEncodedLen)
	            /// Storage: Balances Locks (r:1 w:1)
	            /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	            /// Storage: Balances Freezes (r:1 w:0)
	            /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralEraInfo (r:1 w:1)
	            /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	        fn unstake() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `397`
		      //  Estimated: `4764`
		      // Minimum execution time: 43_000_000 picoseconds.
		      Weight::from_parts(44_000_000, 4764)
			        .saturating_add(T::DbWeight::get().reads(9_u64))
			        .saturating_add(T::DbWeight::get().writes(5_u64))
	        }
	            /// Storage: OcifStaking Halted (r:1 w:0)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	            /// Storage: OcifStaking Ledger (r:1 w:1)
	            /// Proof: OcifStaking Ledger (max_values: None, max_size: Some(265), added: 2740, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CurrentEra (r:1 w:0)
	            /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	            /// Storage: Balances Locks (r:1 w:1)
	            /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	            /// Storage: Balances Freezes (r:1 w:0)
	            /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralEraInfo (r:1 w:1)
	            /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	        fn withdraw_unstaked() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `449`
		      //  Estimated: `4764`
		      // Minimum execution time: 45_000_000 picoseconds.
		      Weight::from_parts(70_000_000, 4764)
			        .saturating_add(T::DbWeight::get().reads(6_u64))
			        .saturating_add(T::DbWeight::get().writes(3_u64))
	        }
	            /// Storage: OcifStaking Halted (r:1 w:0)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralStakerInfo (r:1 w:1)
	            /// Proof: OcifStaking GeneralStakerInfo (max_values: None, max_size: Some(269), added: 2744, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CurrentEra (r:1 w:0)
	            /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CoreEraStake (r:1 w:0)
	            /// Proof: OcifStaking CoreEraStake (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralEraInfo (r:1 w:0)
	            /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	        fn staker_claim_rewards() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `374`
		      //  Estimated: `3734`
		      // Minimum execution time: 24_000_000 picoseconds.
		      Weight::from_parts(26_000_000, 3734)
			        .saturating_add(T::DbWeight::get().reads(5_u64))
			        .saturating_add(T::DbWeight::get().writes(1_u64))
	        }
	            /// Storage: OcifStaking Halted (r:1 w:0)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CurrentEra (r:1 w:0)
	            /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	            /// Storage: OcifStaking CoreEraStake (r:1 w:1)
	            /// Proof: OcifStaking CoreEraStake (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	            /// Storage: OcifStaking GeneralEraInfo (r:1 w:0)
	            /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	        fn core_claim_rewards() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `308`
		      //  Estimated: `3557`
		      // Minimum execution time: 21_000_000 picoseconds.
		      Weight::from_parts(22_000_000, 3557)
			        .saturating_add(T::DbWeight::get().reads(4_u64))
			        .saturating_add(T::DbWeight::get().writes(1_u64))
	        }
	            /// Storage: OcifStaking Halted (r:1 w:1)
	            /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        fn halt_unhalt_pallet() -> Weight {
		      // Proof Size summary in bytes:
		      //  Measured:  `4`
		      //  Estimated: `1486`
		      // Minimum execution time: 8_000_000 picoseconds.
		      Weight::from_parts(9_000_000, 1486)
			        .saturating_add(T::DbWeight::get().reads(1_u64))
			        .saturating_add(T::DbWeight::get().writes(1_u64))
	        }
  }

  // For backwards compatibility and tests.
  impl WeightInfo for () {
	        /// Storage: OcifStaking Halted (r:1 w:0)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        /// Storage: OcifStaking RegisteredCore (r:1 w:1)
	        /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	        /// Storage: System Account (r:1 w:1)
	        /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	        /// The range of component `n` is `[0, 20]`.
	        /// The range of component `d` is `[0, 300]`.
	        /// The range of component `i` is `[0, 100]`.
	    fn register_core(_n: u32, _d: u32, i: u32, ) -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `144`
		  //  Estimated: `3942`
		  // Minimum execution time: 28_000_000 picoseconds.
		  Weight::from_parts(29_525_844, 3942)
			    // Standard Error: 641
			    .saturating_add(Weight::from_parts(2_603, 0).saturating_mul(i.into()))
			    .saturating_add(RocksDbWeight::get().reads(3_u64))
			    .saturating_add(RocksDbWeight::get().writes(2_u64))
	    }
	        /// Storage: OcifStaking Halted (r:1 w:0)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        /// Storage: OcifStaking RegisteredCore (r:1 w:1)
	        /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	        /// The range of component `n` is `[0, 20]`.
	        /// The range of component `d` is `[0, 300]`.
	        /// The range of component `i` is `[0, 100]`.
	    fn change_core_metadata(n: u32, d: u32, i: u32, ) -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `86`
		  //  Estimated: `3942`
		  // Minimum execution time: 13_000_000 picoseconds.
		  Weight::from_parts(13_415_126, 3942)
			    // Standard Error: 2_128
			    .saturating_add(Weight::from_parts(6_775, 0).saturating_mul(n.into()))
			    // Standard Error: 145
			    .saturating_add(Weight::from_parts(1_501, 0).saturating_mul(d.into()))
			    // Standard Error: 436
			    .saturating_add(Weight::from_parts(3_700, 0).saturating_mul(i.into()))
			    .saturating_add(RocksDbWeight::get().reads(2_u64))
			    .saturating_add(RocksDbWeight::get().writes(1_u64))
	    }
	        /// Storage: OcifStaking Halted (r:1 w:0)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        /// Storage: OcifStaking RegisteredCore (r:1 w:1)
	        /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CurrentEra (r:1 w:0)
	        /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralStakerInfo (r:1 w:0)
	        /// Proof: OcifStaking GeneralStakerInfo (max_values: None, max_size: Some(269), added: 2744, mode: MaxEncodedLen)
	        /// Storage: System Account (r:1 w:1)
	        /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	    fn unregister_core() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `268`
		  //  Estimated: `3942`
		  // Minimum execution time: 34_000_000 picoseconds.
		  Weight::from_parts(34_000_000, 3942)
			    .saturating_add(RocksDbWeight::get().reads(5_u64))
			    .saturating_add(RocksDbWeight::get().writes(2_u64))
	    }
	        /// Storage: OcifStaking Halted (r:1 w:0)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        /// Storage: OcifStaking RegisteredCore (r:1 w:0)
	        /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	        /// Storage: OcifStaking Ledger (r:1 w:1)
	        /// Proof: OcifStaking Ledger (max_values: None, max_size: Some(265), added: 2740, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CurrentEra (r:1 w:0)
	        /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CoreEraStake (r:1 w:1)
	        /// Proof: OcifStaking CoreEraStake (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralStakerInfo (r:1 w:1)
	        /// Proof: OcifStaking GeneralStakerInfo (max_values: None, max_size: Some(269), added: 2744, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralEraInfo (r:1 w:1)
	        /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	        /// Storage: Balances Locks (r:1 w:1)
	        /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	        /// Storage: Balances Freezes (r:1 w:0)
	        /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	    fn stake() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `86`
		  //  Estimated: `4764`
		  // Minimum execution time: 46_000_000 picoseconds.
		  Weight::from_parts(47_000_000, 4764)
			    .saturating_add(RocksDbWeight::get().reads(9_u64))
			    .saturating_add(RocksDbWeight::get().writes(5_u64))
	    }
	        /// Storage: OcifStaking Halted (r:1 w:0)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        /// Storage: OcifStaking RegisteredCore (r:1 w:0)
	        /// Proof: OcifStaking RegisteredCore (max_values: None, max_size: Some(477), added: 2952, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CurrentEra (r:1 w:0)
	        /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralStakerInfo (r:1 w:1)
	        /// Proof: OcifStaking GeneralStakerInfo (max_values: None, max_size: Some(269), added: 2744, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CoreEraStake (r:1 w:1)
	        /// Proof: OcifStaking CoreEraStake (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	        /// Storage: OcifStaking Ledger (r:1 w:1)
	        /// Proof: OcifStaking Ledger (max_values: None, max_size: Some(265), added: 2740, mode: MaxEncodedLen)
	        /// Storage: Balances Locks (r:1 w:1)
	        /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	        /// Storage: Balances Freezes (r:1 w:0)
	        /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralEraInfo (r:1 w:1)
	        /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	    fn unstake() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `397`
		  //  Estimated: `4764`
		  // Minimum execution time: 43_000_000 picoseconds.
		  Weight::from_parts(44_000_000, 4764)
			    .saturating_add(RocksDbWeight::get().reads(9_u64))
			    .saturating_add(RocksDbWeight::get().writes(5_u64))
	    }
	        /// Storage: OcifStaking Halted (r:1 w:0)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        /// Storage: OcifStaking Ledger (r:1 w:1)
	        /// Proof: OcifStaking Ledger (max_values: None, max_size: Some(265), added: 2740, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CurrentEra (r:1 w:0)
	        /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	        /// Storage: Balances Locks (r:1 w:1)
	        /// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	        /// Storage: Balances Freezes (r:1 w:0)
	        /// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralEraInfo (r:1 w:1)
	        /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	    fn withdraw_unstaked() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `449`
		  //  Estimated: `4764`
		  // Minimum execution time: 45_000_000 picoseconds.
		  Weight::from_parts(70_000_000, 4764)
			    .saturating_add(RocksDbWeight::get().reads(6_u64))
			    .saturating_add(RocksDbWeight::get().writes(3_u64))
	    }
	        /// Storage: OcifStaking Halted (r:1 w:0)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralStakerInfo (r:1 w:1)
	        /// Proof: OcifStaking GeneralStakerInfo (max_values: None, max_size: Some(269), added: 2744, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CurrentEra (r:1 w:0)
	        /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CoreEraStake (r:1 w:0)
	        /// Proof: OcifStaking CoreEraStake (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralEraInfo (r:1 w:0)
	        /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	    fn staker_claim_rewards() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `374`
		  //  Estimated: `3734`
		  // Minimum execution time: 24_000_000 picoseconds.
		  Weight::from_parts(26_000_000, 3734)
			    .saturating_add(RocksDbWeight::get().reads(5_u64))
			    .saturating_add(RocksDbWeight::get().writes(1_u64))
	    }
	        /// Storage: OcifStaking Halted (r:1 w:0)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CurrentEra (r:1 w:0)
	        /// Proof: OcifStaking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	        /// Storage: OcifStaking CoreEraStake (r:1 w:1)
	        /// Proof: OcifStaking CoreEraStake (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	        /// Storage: OcifStaking GeneralEraInfo (r:1 w:0)
	        /// Proof: OcifStaking GeneralEraInfo (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	    fn core_claim_rewards() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `308`
		  //  Estimated: `3557`
		  // Minimum execution time: 21_000_000 picoseconds.
		  Weight::from_parts(22_000_000, 3557)
			    .saturating_add(RocksDbWeight::get().reads(4_u64))
			    .saturating_add(RocksDbWeight::get().writes(1_u64))
	    }
	        /// Storage: OcifStaking Halted (r:1 w:1)
	        /// Proof: OcifStaking Halted (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	    fn halt_unhalt_pallet() -> Weight {
		  // Proof Size summary in bytes:
		  //  Measured:  `4`
		  //  Estimated: `1486`
		  // Minimum execution time: 8_000_000 picoseconds.
		  Weight::from_parts(9_000_000, 1486)
			    .saturating_add(RocksDbWeight::get().reads(1_u64))
			    .saturating_add(RocksDbWeight::get().writes(1_u64))
	    }
  }
