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

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct AnnualReportStruct<AccountId> {
    pub filehash: H256,
    pub state: AnnualReportStateMask,
    pub signatures: Vec<AccountId>,
    carbon_credits_count: u64
}

impl<AccountId> AnnualReportStruct<AccountId> {
    pub fn new(filehash: H256, carbon_credits_count: u64) -> Self {
        AnnualReportStruct{
            filehash,
            state: REPORT_PROJECT_OWNER_SIGN_PENDING,
            signatures: Vec::new(),
            carbon_credits_count
        }
    }
}