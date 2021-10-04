use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

use crate::standard::Standard;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ProjectStruct<AccountId> {
    pub id: u32,
    pub owner: AccountId,
    pub standard: Standard,
}