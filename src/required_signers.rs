use frame_support::{
    dispatch::Vec,
};
use pallet_evercity_accounts::accounts::RoleMask;

pub type RequiredSigner<AccountId> = (AccountId, RoleMask);