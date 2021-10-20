use crate::standard::Standard;
use crate::project::ProjectId;

pub struct CarbonCreditBatch {
    standard: Standard,
    project_id: ProjectId, 
    // timestamp: Timestamp,
    count: u64,
}

pub struct CarbonCreditWarehouse<AccountId> {
    pub owner: AccountId,
    pub carbon_credit_batches: Vec<CarbonCreditBatch>,
}