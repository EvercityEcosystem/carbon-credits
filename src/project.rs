use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use fixed_hash::construct_fixed_hash;
use crate::standard::Standard;

construct_fixed_hash! {
    /// 256 bit hash type for signing files
    #[derive(Encode, Decode)]
    pub struct H256(32);
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ProjectStruct<AccountId> {
    pub owner: AccountId,
    pub id: u32,
    pub standard: Standard,
    pub status: ProjectStatus,
    pub filehash: H256,
}

impl<AccountId> ProjectStruct<AccountId> {
    /// constructor for project
    pub fn new(owner: AccountId, id: u32, standard: Standard, filehash: &H256) -> Self {
        ProjectStruct{
            owner,
            id,
            standard,
            status: ProjectStatus::default(), 
            filehash: *filehash
        }
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug)]
pub enum ProjectStatus {
    Preparing,
    Registration,
    Issuance,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::Preparing
    }
}