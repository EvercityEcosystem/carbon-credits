use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

pub const MASTER_ROLE_MASK: u8 = 1u8;
pub const PROJECT_OWNER_ROLE_MASK: u8 = 2u8;
pub const AUDITOR_ROLE_MASK: u8 = 4u8;
pub const STANDARD_ROLE_MASK: u8 = 8u8;
pub const INVESTOR_ROLE_MASK: u8 = 16u8;
pub const REGISTRY_ROLE_MASK: u8 = 32u8;

pub const ALL_ROLES_MASK: u8 = MASTER_ROLE_MASK
    | PROJECT_OWNER_ROLE_MASK
    | AUDITOR_ROLE_MASK
    | STANDARD_ROLE_MASK
    | AUDITOR_ROLE_MASK
    | INVESTOR_ROLE_MASK
    | REGISTRY_ROLE_MASK;

#[inline]
pub const fn is_roles_correct(roles: u8) -> bool {
    // max value of any roles combinations
    roles <= ALL_ROLES_MASK && roles > 0
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct CarbonCreditAccountStruct {
    pub roles: u8,
}

impl CarbonCreditAccountStruct {
    pub fn new(roles: u8) -> Self {
        CarbonCreditAccountStruct{
            roles
        }
    }
}