use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

use crate::standard::Standard;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ProjectStruct<AccountId> {
    pub owner: AccountId,
    pub id: u32,
    pub standard: Standard,
}

impl<AccountId> ProjectStruct<AccountId> {
    /// constructor for project
    pub fn new(owner: AccountId, id: u32, standard: Standard) -> Self {
        ProjectStruct{
            owner,
            id,
            standard
        }
    }
}