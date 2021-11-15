use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use crate::standard::Standard;
use crate::annual_report::*;
use pallet_evercity_filesign::FileId;
use frame_support::sp_std::{
    cmp::{
        PartialEq}, 
};
use crate::required_signers::RequiredSigner;

pub type ProjectStateMask = u16;
pub const PROJECT_OWNER_SIGN_PENDING: ProjectStateMask = 1;
pub const AUDITOR_SIGN_PENDING: ProjectStateMask = 2;
pub const STANDARD_SIGN_PENDING: ProjectStateMask = 4;
pub const INVESTOR_SIGN_PENDING: ProjectStateMask = 8;
pub const REGISTRY_SIGN_PENDING: ProjectStateMask = 16;
pub const REGISTERED: ProjectStateMask = 32;

pub type ProjectId = u32;

/// Main struct for projects
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct ProjectStruct<AccountId, Moment, Balance> where AccountId: PartialEq + Clone, Moment: pallet_timestamp::Config, Balance: Clone {
    pub owner: AccountId,
    pub id: ProjectId,
    pub status: ProjectStatus,
    pub state: ProjectStateMask,
    pub file_id: FileId,
    pub annual_reports: Vec<AnnualReportStruct<AccountId, Moment, Balance>>,
    required_signers: Vec<RequiredSigner<AccountId>>,
    standard: Standard,
}

impl<AccountId, Moment, Balance> ProjectStruct<AccountId, Moment, Balance> where AccountId: PartialEq + Clone, Moment: pallet_timestamp::Config, Balance: Clone {
    /// constructor for project
    pub fn new(owner: AccountId, id: u32, standard: Standard, file_id: FileId) -> Self {
        ProjectStruct{
            file_id, 
            owner,
            id,
            standard,
            status: ProjectStatus::default(), 
            state: PROJECT_OWNER_SIGN_PENDING,
            annual_reports: Vec::new(),
            required_signers: Vec::new(),
        }
    }

    // Standart must be guaranted immutable for lifetime of the progect on register and issuance step 
    pub fn get_standard(&self) -> &Standard {
        &self.standard
    }

    pub fn assign_required_signer(&mut self, signer: RequiredSigner<AccountId>) {
        self.required_signers.push(signer);
    }

    pub fn is_required_signer(&self, signer: RequiredSigner<AccountId>) -> bool {
        self.required_signers.iter().any(|(acc, role)| *acc == signer.0 && *role == signer.1)
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ProjectStatus {
    PREPARING,
    REGISTRATION,
    ISSUANCE,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::PREPARING
    }
}