

// pub enum State {
//     ProjectOwnerSignPending,
//     AuditorSignPending,
//     StandardSignPending,
//     InvestorSignPending,
//     RegistrySignPending,
// }

pub type StateMask = u16;

pub const PROJECT_OWNER_SIGN_PENDING: StateMask = 1;
pub const AUDITOR_SIGN_PENDING: StateMask = 2;
pub const STANDARD_SIGN_PENDING: StateMask = 4;
pub const INVESTOR_SIGN_PENDING: StateMask = 8;
pub const REGISTRY_SIGN_PENDING: StateMask = 16;
pub const REGISTERED: StateMask = 32;