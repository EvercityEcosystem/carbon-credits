use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};


pub const PROJECT_OWNER: u8 = 1u8;
pub const PROJECT_DEVELOPER: u8 = 2u8;
pub const STANDART: u8 = 4u8;
pub const INVESTOR_ROLE_MASK: u8 = 8u8;
pub const AUDITOR_ROLE_MASK: u8 = 16u8;
pub const REGISTRY: u8 = 32u8;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct CarbonCreditAccountStruct<Moment> {
    pub roles: u8,
    // #[codec(compact)]
    // pub identity: u64,
    #[codec(compact)]
    pub create_time: Moment,
}

pub type CarbonCreditAccountStructOf<T> =
    CarbonCreditAccountStruct<<T as pallet_timestamp::Config>::Moment>;