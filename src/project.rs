use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};

use crate::standard::Standard;
use crate::annual_report::*;
use crate::file_hash::*;
use frame_support::sp_std::{
    cmp::{
        PartialEq}, 
};

pub type ProjectStateMask = u16;
pub const PROJECT_OWNER_SIGN_PENDING: ProjectStateMask = 1;
pub const AUDITOR_SIGN_PENDING: ProjectStateMask = 2;
pub const STANDARD_SIGN_PENDING: ProjectStateMask = 4;
pub const INVESTOR_SIGN_PENDING: ProjectStateMask = 8;
pub const REGISTRY_SIGN_PENDING: ProjectStateMask = 16;
pub const REGISTERED: ProjectStateMask = 32;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct ProjectStruct<AccountId> where AccountId: PartialEq {
    pub owner: AccountId,
    pub id: u32,
    pub status: ProjectStatus,
    pub state: ProjectStateMask,
    pub document_versions: Vec<ProjectDocument>,
    pub signatures: Vec<AccountId>,
    pub annual_reports: Vec<AnnualReportStruct<AccountId>>,
    standard: Standard,
}

impl<AccountId> ProjectStruct<AccountId> where AccountId: PartialEq {
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

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
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