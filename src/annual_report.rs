use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use fixed_hash::construct_fixed_hash;

pub type AnnualReportState = u16;
pub const REPORT_PROJECT_OWNER_SIGN_PENDING: AnnualReportState = 1;
pub const REPORT_AUDITOR_SIGN_PENDING: AnnualReportState = 2;
pub const REPORT_STANDARD_SIGN_PENDING: AnnualReportState = 4;
pub const REPORT_INVESTOR_SIGN_PENDING: AnnualReportState = 8;
pub const REPORT_REGISTRY_SIGN_PENDING: AnnualReportState = 16;
pub const REPORT_ISSUED: AnnualReportState = 32;

construct_fixed_hash! {
    /// 256 bit hash type for signing files
    #[derive(Encode, Decode)]
    pub struct H256(32);
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
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