use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use crate::file_hash::*;

pub type AnnualReportState = u16;
pub const REPORT_PROJECT_OWNER_SIGN_PENDING: AnnualReportState = 1;
pub const REPORT_AUDITOR_SIGN_PENDING: AnnualReportState = 2;
pub const REPORT_STANDARD_SIGN_PENDING: AnnualReportState = 4;
pub const REPORT_INVESTOR_SIGN_PENDING: AnnualReportState = 8;
pub const REPORT_REGISTRY_SIGN_PENDING: AnnualReportState = 16;
pub const REPORT_ISSUED: AnnualReportState = 32;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct AnnualReportStruct {
    pub filehash: H256,
}

impl AnnualReportStruct {
    pub fn new(filehash: H256) -> Self {
        AnnualReportStruct{
            filehash
        }
    }
}