use super::pallet::*;
use crate::util::derive_core_account;
use frame_support::pallet_prelude::*;
use frame_system::{ensure_signed, pallet_prelude::*};
use primitives::{CoreInfo, OneOrPercent};
use sp_arithmetic::traits::{CheckedAdd, One};
use sp_std::{convert::TryInto, vec::Vec};

pub type CoreIndexOf<T> = <T as Config>::CoreId;

pub type CoreMetadataOf<T> = BoundedVec<u8, <T as Config>::MaxMetadata>;

impl<T: Config> Pallet<T> {
    /// Create IP Set
    pub(crate) fn inner_create_core(
        owner: OriginFor<T>,
        metadata: Vec<u8>,
        execution_threshold: OneOrPercent,
        default_asset_weight: OneOrPercent,
        default_permission: bool,
    ) -> DispatchResult {
        NextCoreId::<T>::try_mutate(|next_id| -> DispatchResult {
            let creator = ensure_signed(owner.clone())?;

            let bounded_metadata: BoundedVec<u8, T::MaxMetadata> = metadata
                .try_into()
                .map_err(|_| Error::<T>::MaxMetadataExceeded)?;

            // Increment counter
            let current_id = *next_id;
            *next_id = next_id
                .checked_add(&One::one())
                .ok_or(Error::<T>::NoAvailableCoreId)?;

            // Generate new `AccountId` to represent new IP Set being created
            let core_account = derive_core_account::<
                T,
                <T as Config>::CoreId,
                <T as frame_system::Config>::AccountId,
            >(current_id);

            // Send IP Set `creator` 1,000,000 "IPT0" tokens
            // Token has 6 decimal places: 1,000,000 / 10^6 = 1 IPTO token
            // This allows for token divisiblity
            Balance::<T>::insert::<
                (<T as Config>::CoreId, Option<<T as Config>::CoreId>),
                T::AccountId,
                <T as Config>::Balance,
            >((current_id, None), creator, 1_000_000u128.into());

            let info = CoreInfo {
                account: core_account.clone(),
                metadata: bounded_metadata,

                supply: 1_000_000u128.into(),

                execution_threshold: execution_threshold,
                default_asset_weight: default_asset_weight,
                default_permission: default_permission,
            };

            // Update core IPS storage
            CoreStorage::<T>::insert(current_id, info);
            CoreByAccount::<T>::insert(core_account.clone(), current_id);

            Self::deposit_event(Event::CoreCreated {
                core_account,
                core_id: current_id,
            });

            Ok(())
        })
    }
}