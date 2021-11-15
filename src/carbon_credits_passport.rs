use crate::project::ProjectId;
use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

/// Passport, that prooves, that an asset is a carbon credit asset
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsPassport<AssetId>{
    asset_id: AssetId,
    project_id: ProjectId,
    annual_report_index: u64,
}

impl<AssetId> CarbonCreditsPassport<AssetId> {
    pub fn new(asset_id: AssetId, project_id: ProjectId, annual_report_index: usize) -> Self {
        let annual_report_index_inner = annual_report_index as u64;

        CarbonCreditsPassport{
            asset_id,
            project_id,
            annual_report_index: annual_report_index_inner,
        }
    }

    pub fn get_project_id(&self) -> ProjectId { 
        self.project_id
    }

    pub fn get_asset_id_ref(&self) -> &AssetId { 
        &self.asset_id
    }

    pub fn get_annual_report_index(&self) -> u64 {
        self.annual_report_index
    }

    pub fn get_last_report_index(&self) -> usize { 
        self.annual_report_index as usize
    }
}