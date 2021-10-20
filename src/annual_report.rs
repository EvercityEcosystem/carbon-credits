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

pub type AnnualReportStruct<AccountId, T> = AnnualReportStructT<AccountId, <T as pallet_timestamp::Config>::Moment>;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct AnnualReportStructT<AccountId, Moment> {
    pub filehash: H256,
    pub state: AnnualReportStateMask,
    pub signatures: Vec<AccountId>,
    #[codec(compact)]
    create_time: Moment,
    carbon_credits_count: u64,
    carbon_credits_released: bool,
}

impl<AccountId, Moment> AnnualReportStructT<AccountId, Moment> {
    pub fn new(filehash: H256, carbon_credits_count: u64, create_time: Moment) -> Self {
        AnnualReportStructT{
            filehash,
            state: REPORT_PROJECT_OWNER_SIGN_PENDING,
            signatures: Vec::new(),
            carbon_credits_count,
            carbon_credits_released: false,
            create_time,
        }
    }
}