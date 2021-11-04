// use crate::standard::Standard;
use crate::project::ProjectId;
use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsPassport<AssetId>{
    asset_id: AssetId,
    project_id: ProjectId,
    annual_report_index: u64,
}

impl<AssetId> CarbonCreditsPassport<AssetId> {
    pub fn new(asset_id: AssetId, project_id: ProjectId, annual_report_index: u64) -> Self {
        CarbonCreditsPassport{
            asset_id,
            project_id,
            annual_report_index,
        }
    }
}