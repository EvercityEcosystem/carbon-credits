use crate::standard::Standard;
use crate::project::ProjectId;

pub struct CarbonCreditBatch<Moment> where Moment: pallet_timestamp::Config {
    pub standard: Standard,
    pub project_id: ProjectId, 
    pub timestamp: Moment,
    pub count: u64,
}

pub struct CarbonCreditWarehouse<AccountId, Moment> where Moment: pallet_timestamp::Config {
    pub owner: AccountId,
    pub carbon_credit_batches: Vec<CarbonCreditBatch<Moment>>,
}