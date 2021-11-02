use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use crate::file_hash::*;

pub type AnnualReportStateMask = u16;
pub const REPORT_PROJECT_OWNER_SIGN_PENDING: AnnualReportStateMask = 1;
pub const REPORT_AUDITOR_SIGN_PENDING: AnnualReportStateMask = 2;
pub const REPORT_STANDARD_SIGN_PENDING: AnnualReportStateMask = 4;
pub const REPORT_INVESTOR_SIGN_PENDING: AnnualReportStateMask = 8;
pub const REPORT_REGISTRY_SIGN_PENDING: AnnualReportStateMask = 16;
pub const REPORT_ISSUED: AnnualReportStateMask = 32;

pub type AnnualReportStruct<AccountId, T, Balance> = AnnualReportStructT<AccountId, <T as pallet_timestamp::Config>::Moment, Balance>;

pub type CarbonCreditsBalance = u32;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct AnnualReportStructT<AccountId, Moment, Balance> where Balance: Clone {
    pub filehash: H256,
    pub state: AnnualReportStateMask,
    pub signatures: Vec<AccountId>,
    #[codec(compact)]
    create_time: Moment,
    carbon_credits_count: Balance,
    carbon_credits_released: bool,
}

impl<AccountId, Moment, Balance> AnnualReportStructT<AccountId, Moment, Balance> where Balance: Clone {
    pub fn new(filehash: H256, carbon_credits_count: Balance, create_time: Moment) -> Self {
        AnnualReportStructT{
            filehash,
            state: REPORT_PROJECT_OWNER_SIGN_PENDING,
            signatures: Vec::new(),
            carbon_credits_count,
            carbon_credits_released: false,
            create_time,
        }
    }

    pub fn is_carbon_credits_released(&self) -> bool {
        self.carbon_credits_released
    }

    pub fn set_carbon_credits_released(&mut self) {
        self.carbon_credits_released = true;
    }

    pub fn carbon_credits_count(&self) -> Balance {
        self.carbon_credits_count.clone()
    }
}