use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use fixed_hash::construct_fixed_hash;
use crate::standard::Standard;
use crate::state::*;
use crate::annual_report::*;

construct_fixed_hash! {
    /// 256 bit hash type for signing files
    #[derive(Encode, Decode)]
    pub struct H256(32);
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ProjectStruct<AccountId> {
    pub owner: AccountId,
    pub id: u32,
    pub status: ProjectStatus,
    pub state: ProjectStateMask,
    pub document_versions: Vec<ProjectDocument>,
    pub signatures: Vec<AccountId>,
    pub annual_reports: Vec<AnnualReportStruct>,
    standard: Standard,
}

impl<AccountId> ProjectStruct<AccountId> {
    /// constructor for project
    pub fn new(owner: AccountId, id: u32, standard: Standard, filehash: &H256) -> Self {
        ProjectStruct{
            owner,
            id,
            standard,
            status: ProjectStatus::default(), 
            state: PROJECT_OWNER_SIGN_PENDING,
            document_versions: vec![ProjectDocument::new(filehash)],
            signatures: Vec::new(),
            annual_reports: Vec::new(),
        }
    }

    // Standart must be guaranted immutable for lifetime of the progect on register and issuance step 
    pub fn get_standard(&self) -> &Standard {
        &self.standard
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ProjectDocument {
    pub filehash: H256,
}

impl ProjectDocument {
    fn new(filehash: &H256) -> Self {
        Self {filehash: *filehash}
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum ProjectStatus {
    Preparing,
    Registration,
    Issuance,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::Preparing
    }
}