use frame_support::{
    dispatch::Vec,
};
use pallet_evercity_accounts::accounts::RoleMask;

pub type RequiredSigners<AccountId> = Vec<(AccountId, RoleMask)>;