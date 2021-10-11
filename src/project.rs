use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use fixed_hash::construct_fixed_hash;
use crate::standard::Standard;
use crate::state::*;

construct_fixed_hash! {
    /// 256 bit hash type for signing files
    #[derive(Encode, Decode)]
    pub struct H256(32);
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct ProjectStruct<AccountId> {
    pub owner: AccountId,
    pub id: u32,
    pub standard: Standard,
    pub status: ProjectStatus,
    pub state: StateMask,
    pub document_versions: Vec<ProjectDocument>,
    pub signatures: Vec<AccountId>,
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
            signatures: Vec::new()
        }
    }

    // pub fn change_project_state(&mut self, signaturer: AccountId) -> Result<(), ProjectError> {
    //     match &mut self.standard {
    //         GoldStandard  => {
    //             match self.state {
    //                 PROJECT_OWNER_SIGN_PENDING => {
    //                     //check that is project owner
    //                     // todo!("check that is project owner");
    //                     // crate::accounts::

    //                     self.state = AUDITOR_SIGN_PENDING;
    //                 },
    //                 _ => return Err(ProjectError::InvalidState)
    //             }

    //             Ok(())
    //         },
    //         _ => Err(ProjectError::InvalidStandard),
    //     }
    // }
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


// pub enum ProjectError {
//     InvalidStandard,
//     NotAnOwner,
//     InvalidState
// }