#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
    decl_event,
    dispatch::{
        DispatchResult,
    },
};
use frame_system::{
    ensure_signed,
};
use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
};
use pallet_evercity_accounts as accounts;
use project::{ProjectStruct, ProjectId};
use standard::Standard;
use crate::file_hash::*;
use pallet_evercity_accounts::accounts::RoleMask;

pub mod standard;
pub mod project;
pub mod annual_report;
pub mod file_hash;
pub mod required_signers;

#[cfg(test)]    
pub mod tests;

pub trait Config: frame_system::Config + pallet_evercity_accounts::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        ProjectById
            get(fn project_by_id):
            map hasher(blake2_128_concat) u32 => Option<ProjectStruct<T::AccountId>>;

        LastID: ProjectId;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        // Project Events
        ProjectCreated(AccountId, ProjectId),
        ProjectSubmited(AccountId, ProjectId),    
        ProjectRegistered(AccountId, ProjectId),
        ProjectSignedByAduitor(AccountId, ProjectId),
        ProjectSignedByStandard(AccountId, ProjectId),
        ProjectSignedByRegistry(AccountId, ProjectId),

        // Annual Report events
        AnnualReportCreated(AccountId, ProjectId),
        AnnualReportSubmited(AccountId, ProjectId),
        AnnualReportSignedByAuditor(AccountId, ProjectId),
        AnnualReportSignedByStandard(AccountId, ProjectId),
        AnnualReportSignedByRegistry(AccountId, ProjectId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        // Account errors:
        AccountNotAuthorized,
        AccountNotAuditor,
        AccountNotOwner,
        AccountNotStandard,
        AccountNotRegistry,
        AccountNotInvestor,
        AccountToAddAlreadyExists,
        AccountRoleParamIncorrect,
        AccountNotExist,

        InvalidAction,
        InvalidState,
        InvalidStandard,
        ProjectNotExist,
        ProjectNotRegistered,
        NoAnnualReports,
        NotIssuedAnnualReportsExist,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn create_project(origin, standard: Standard, filehash: H256) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::impl_create_project(caller, standard, &filehash)?;
            Ok(())
        }

        #[weight = 10_000]
        pub fn assign_signer(origin, signer: T::AccountId, role: RoleMask) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            // Self::impl_sign_project(caller, proj_id)?;
            todo!();
            Ok(())
        }

        #[weight = 10_000]
        pub fn sign_project(origin, proj_id: u32) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::impl_sign_project(caller, proj_id)?;
            Ok(())
        }

        #[weight = 10_000]
        pub fn create_annual_report(origin, project_id: u32, filehash: H256, carbon_credits_count: u64) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::impl_create_annual_report(caller, project_id, &filehash, carbon_credits_count)?;
            Ok(())
        }

        #[weight = 10_000]
        pub fn sign_last_annual_report(origin, project_id: u32) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::impl_sign_annual_report(caller, project_id)?;
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    pub fn impl_create_project(caller: T::AccountId, standard: Standard, filehash: &H256) -> DispatchResult {
        // check if caller has CC_PROJECT_OWNER role
        ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);

        let new_id = LastID::get() + 1;
        let new_project = ProjectStruct::<<T as frame_system::Config>::AccountId>::new(caller.clone(), new_id, standard, filehash);
        <ProjectById<T>>::insert(new_id, new_project);
        LastID::mutate(|x| *x = x.checked_add(1).unwrap());
        // SendEvent
        Self::deposit_event(RawEvent::ProjectCreated(caller, new_id));
        Ok(())
    }

    fn impl_sign_project(caller: T::AccountId, proj_id: u32) -> DispatchResult {
        let mut event_opt: Option<Event<T>> = None;
        ProjectById::<T>::try_mutate(
            proj_id, |project_to_mutate| -> DispatchResult {
                ensure!(project_to_mutate.is_some(), Error::<T>::ProjectNotExist);
                Self::change_project_state(&mut project_to_mutate.as_mut().unwrap(), caller, &mut event_opt)?;
                Ok(())
         })?;
        if let Some(event) = event_opt {
            Self::deposit_event(event);
        }
        Ok(())
    }

    fn impl_create_annual_report(caller: T::AccountId, proj_id: ProjectId, filehash: &H256, carbon_credits_count: u64) -> DispatchResult {
        ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
        ProjectById::<T>::try_mutate(
            proj_id, |project_to_mutate| -> DispatchResult {
                ensure!(project_to_mutate.is_some(), Error::<T>::ProjectNotExist);
                ensure!(project_to_mutate.as_ref().unwrap().owner == caller, Error::<T>::AccountNotOwner);
                ensure!(project_to_mutate.as_ref().unwrap().state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
                ensure!(project_to_mutate.as_ref().unwrap().annual_reports.iter()
                            .all(|x| x.state == annual_report::REPORT_ISSUED),
                    Error::<T>::NotIssuedAnnualReportsExist
                );
                project_to_mutate.as_mut().unwrap().annual_reports.push(annual_report::AnnualReportStruct::new(*filehash, carbon_credits_count));
                Ok(())
         })?;
        // SendEvent
        Self::deposit_event(RawEvent::AnnualReportCreated(caller, proj_id));
        Ok(())
    }

    fn change_project_state(project: &mut ProjectStruct<T::AccountId>, caller: T::AccountId, event: &mut Option<Event<T>>) -> DispatchResult {
        match &mut project.get_standard() {
            // Project Owner submits PDD (changing status to Registration) => 
            // => Auditor Approves PDD => Standard Certifies PDD => Registry Registers PDD (changing status to Issuance)
            Standard::GOLD_STANDARD  => {
                match project.state {
                    project::PROJECT_OWNER_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                        ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                        project.state = project::AUDITOR_SIGN_PENDING;
                        project.status = project::ProjectStatus::REGISTRATION;
                        project.signatures.push(caller.clone());
                        *event = Some(RawEvent::ProjectSubmited(caller, project.id));
                    },
                    project::AUDITOR_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                        project.state = project::STANDARD_SIGN_PENDING;
                        project.signatures.push(caller.clone());
                        *event = Some(RawEvent::ProjectSignedByAduitor(caller, project.id));
                    },
                    project::STANDARD_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                        project.state = project::REGISTRY_SIGN_PENDING;
                        project.signatures.push(caller.clone());
                        *event = Some(RawEvent::ProjectSignedByStandard(caller, project.id));
                    },
                    project::REGISTRY_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                        project.state = project::REGISTERED;
                        project.status = project::ProjectStatus::ISSUANCE;
                        project.signatures.push(caller.clone());
                        *event = Some(RawEvent::ProjectSignedByRegistry(caller, project.id));
                    },
                    _ => ensure!(false, Error::<T>::InvalidState)
                }
                Ok(())
            }
        }
    }

    fn impl_sign_annual_report(caller: T::AccountId, proj_id: ProjectId) -> DispatchResult {
        let mut event_opt: Option<Event<T>> = None;
        ProjectById::<T>::try_mutate(
            proj_id, |project_to_mutate| -> DispatchResult {
                ensure!(project_to_mutate.is_some(), Error::<T>::ProjectNotExist);
                ensure!(project_to_mutate.as_ref().unwrap().annual_reports.last().is_some(), Error::<T>::NoAnnualReports);
                Self::change_project_annual_report_state(&mut project_to_mutate.as_mut().unwrap(), caller, &mut event_opt)?;
                Ok(())
        })?;
        if let Some(event) = event_opt {
            Self::deposit_event(event);
        }
        Ok(())
    }

    fn change_project_annual_report_state(project: &mut ProjectStruct<T::AccountId>, caller: T::AccountId, event: &mut Option<Event<T>>) -> DispatchResult {
        let standard = project.get_standard().clone();
        let owner = project.owner.clone();
        
        let report = project.annual_reports.last_mut().unwrap();
        match standard {
            // Project Owner sends report for verification =>  Auditor provides and submits verification report => 
            // Standard Approves carbon credit issuance => Registry issues carbon credits
            Standard::GOLD_STANDARD  => {
                match report.state {
                    annual_report::REPORT_PROJECT_OWNER_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                        ensure!(owner == caller, Error::<T>::AccountNotOwner);
                        report.state = annual_report::REPORT_AUDITOR_SIGN_PENDING;
                        report.signatures.push(caller.clone());
                        *event = Some(RawEvent::AnnualReportSubmited(caller, project.id));
                    },
                    annual_report::REPORT_AUDITOR_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                        report.state = annual_report::REPORT_STANDARD_SIGN_PENDING;
                        report.signatures.push(caller.clone());
                        *event = Some(RawEvent::AnnualReportSignedByAuditor(caller, project.id));
                    },
                    annual_report::REPORT_STANDARD_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                        report.state = annual_report::REPORT_REGISTRY_SIGN_PENDING;
                        report.signatures.push(caller.clone());
                        *event = Some(RawEvent::AnnualReportSignedByStandard(caller, project.id));
                    },
                    annual_report::REPORT_REGISTRY_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                        report.state = annual_report::REPORT_ISSUED;
                        report.signatures.push(caller.clone());
                        *event = Some(RawEvent::AnnualReportSignedByRegistry(caller, project.id));

                        /*
                            !!!!!!!!!!!_TODO_!!!!!!!!!!!
                            HERE ISSUE CARBON CREDITS
                        */
                    },
                    _ => ensure!(false, Error::<T>::InvalidState)
                }
                Ok(())
            },
        }
    }

    #[cfg(test)]
    pub fn get_proj_by_id(id: ProjectId) -> Option<ProjectStruct<T::AccountId>> {
        ProjectById::<T>::get(id)
    }
}