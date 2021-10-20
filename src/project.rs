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
use crate::required_signers::RequiredSigners;

pub type ProjectStateMask = u16;
pub const PROJECT_OWNER_SIGN_PENDING: ProjectStateMask = 1;
pub const AUDITOR_SIGN_PENDING: ProjectStateMask = 2;
pub const STANDARD_SIGN_PENDING: ProjectStateMask = 4;
pub const INVESTOR_SIGN_PENDING: ProjectStateMask = 8;
pub const REGISTRY_SIGN_PENDING: ProjectStateMask = 16;
pub const REGISTERED: ProjectStateMask = 32;

pub type ProjectId = u32;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct ProjectStruct<AccountId> where AccountId: PartialEq {
    pub owner: AccountId,
    pub id: ProjectId,
    pub status: ProjectStatus,
    pub state: ProjectStateMask,
    pub document_versions: Vec<ProjectDocument>,
    pub signatures: Vec<AccountId>,
    pub annual_reports: Vec<AnnualReportStruct<AccountId>>,
    pub required_signers: RequiredSigners<AccountId>,
    standard: Standard,
}

impl<AccountId> ProjectStruct<AccountId> where AccountId: PartialEq {
    /// constructor for project
    pub fn new(owner: AccountId, id: u32, standard: Standard, filehash: &H256) -> Self {

        // Add first version of document
        let mut document_versions = Vec::with_capacity(1);
        document_versions.push(ProjectDocument::new(filehash));

        ProjectStruct{
            owner,
            id,
            standard,
            status: ProjectStatus::default(), 
            state: PROJECT_OWNER_SIGN_PENDING,
            document_versions,
            signatures: Vec::new(),
            annual_reports: Vec::new(),
            required_signers: Vec::new(),
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