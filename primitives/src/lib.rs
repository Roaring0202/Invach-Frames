#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};

/// IPS Id type
pub type IpsId = u64;
/// IPT Id type
pub type IptId = u64;
/// IPO id type
pub type IpoId = u64;
/// DEV id type
pub type DevId = u64;

/// IPS info
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug)]
pub struct IpsInfo<AccountId, Data, IpsMetadataOf> {
    /// IPS owner
    pub owner: AccountId,
    /// IPS metadata
    pub metadata: IpsMetadataOf,
    /// IPS Properties
    pub data: Data,
}

/// IPT Info
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug)]
pub struct IptInfo<AccountId, Data, IptMetadataOf> {
    /// IPT owner
    pub owner: AccountId,
    /// IPT metadata
    pub metadata: IptMetadataOf,
    /// IPT data
    pub data: Data,
}

/// IPO Info
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen)]
pub struct IpoInfo<AccountId, Data, IpoMetadataOf> {
    /// IPO metadata
    pub metadata: IpoMetadataOf,
    /// Total issuance for the IPO
    pub total_issuance: u64,
    /// IPO owner
    pub owner: AccountId,
    /// IPO Properties
    pub data: Data,
    /// Binding Properties
    pub is_bond: bool,
}

/// DEV Info
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct DevInfo<
    AccountId,
    DevMetadataOf,
    IpsId,
    DevUsers,
    Allocation,
    DevInteractions,
    DevTerms,
    DevMilestones,
> {
    /// DEV owner
    pub owner: AccountId,
    /// DEV metadata
    pub metadata: DevMetadataOf,
    /// Id of the IPS that the DEV refers to
    pub ips_id: IpsId,
    /// IPO allocations for DEV
    pub users: DevUsers,
    /// Total issuance of IPO for this DEV (if this is 100 the ipo allocations will be percentages)
    pub total_issuance: Allocation,
    /// DEV interactions
    pub interactions: DevInteractions,
    /// Terms of the DEV
    pub terms: DevTerms,
    /// Dev Milestones,
    pub milestones: DevMilestones,
    /// DEV post as joinable
    pub is_joinable: bool,
}

/// Dev User
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct DevUser<Allocation, Role> {
    pub allocation: Allocation,
    pub role: Role,
}
