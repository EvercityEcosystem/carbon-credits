// use crate::standard::Standard;
use crate::project::ProjectId;
use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};

// pub struct CarbonCreditBatch<Moment> where Moment: pallet_timestamp::Config {
//     pub standard: Standard,
//     pub project_id: ProjectId, 
//     pub timestamp: Moment,
//     pub count: u64,
// }

// pub struct CarbonCreditWarehouse<AccountId, Moment> where Moment: pallet_timestamp::Config {
//     pub owner: AccountId,
//     pub carbon_credit_batches: Vec<CarbonCreditBatch<Moment>>,
// }

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsRegistry<AssetId> {
    asset_id: AssetId,
    project_id: ProjectId,
    annual_report_index: u64,
}
