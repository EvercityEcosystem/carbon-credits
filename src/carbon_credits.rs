// use crate::standard::Standard;
use crate::project::ProjectId;
use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use std::ops::{Add, Sub};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsPassport<AssetId, Balance> where Balance: Clone + Default + Add<Output = Balance> + Sub {
    asset_id: AssetId,
    project_id: ProjectId,
    annual_report_index: u64,
    burned_amount: Balance
}

impl<AssetId, Balance> CarbonCreditsPassport<AssetId, Balance> where Balance: Clone + Default + Add<Balance, Output = Balance> + Sub {
    pub fn new(asset_id: AssetId, project_id: ProjectId, annual_report_index: u64) -> Self {
        CarbonCreditsPassport{
            asset_id,
            project_id,
            annual_report_index,
            burned_amount: Balance::default(),
        }
    }

    pub fn increment_burn_amount(&mut self, amount: Balance) {
        self.burned_amount = self.burned_amount.clone().add(amount);
    } 
}