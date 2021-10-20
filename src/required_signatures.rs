use frame_support::{
    dispatch::Vec,
};
use pallet_evercity_accounts::accounts::RoleMask;

pub type RequiredSignatures<AccountId> = Vec<(AccountId, RoleMask)>;