use crate::standard::Standard;
use crate::project::ProjectId;

pub struct CarbonCredit {
    standard: Standard,
    project_id: ProjectId, 
    // timestamp: Timestamp,
}

pub struct CarbonCreditWarehouse<AccountId> {
    pub owner: AccountId,
    pub credits: Vec<CarbonCredit>,
}