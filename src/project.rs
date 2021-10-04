use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ProjectStruct<AccountId> {
    pub id: u32,
    pub owner: AccountId,
}