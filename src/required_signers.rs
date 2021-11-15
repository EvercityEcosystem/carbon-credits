use pallet_evercity_accounts::accounts::RoleMask;

/// Accounts and it's role
pub type RequiredSigner<AccountId> = (AccountId, RoleMask);