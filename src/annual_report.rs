use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct AnnualReportStruct {
}

impl AnnualReportStruct {
    pub fn new() -> Self {
        AnnualReportStruct{}
    }
} 