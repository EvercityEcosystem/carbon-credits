#![cfg_attr(not(feature = "std"), no_std)]

pub mod standard;
pub mod project;
pub mod annual_report;
pub mod required_signers;
pub mod carbon_credits_passport;
pub mod burn_certificate;

#[cfg(test)]    
pub mod tests;

use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
    decl_event,
    dispatch::{
        DispatchResult,
        Vec,
    },
    traits::UnfilteredDispatchable,
};
use frame_system::{
    ensure_signed,
};
use sp_runtime::traits::StaticLookup;
use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
};
pub use pallet_evercity_assets::weights::WeightInfo;
pub use pallet_evercity_assets as pallet_assets;

use pallet_evercity_accounts as accounts;
use project::{ProjectStruct, ProjectId};
use standard::Standard;
use pallet_evercity_filesign::{FileId};
use pallet_evercity_accounts::accounts::RoleMask;
use carbon_credits_passport::CarbonCreditsPassport;
use burn_certificate::CarbonCreditsBurnCertificate;

type Timestamp<T> = pallet_timestamp::Module<T>;
 
pub trait Config: 
    frame_system::Config + 
    pallet_evercity_accounts::Config + 
    pallet_timestamp::Config + 
    pallet_assets::Config + 
    pallet_evercity_filesign::Config {
        type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

type AssetId<T> = <T as pallet_assets::Config>::AssetId;

// Pallet Storage
decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        /// Main storage for projects
        ProjectById
            get(fn project_by_id):
            map hasher(blake2_128_concat) u32 => Option<ProjectStruct<T::AccountId, T, T::Balance>>;

        /// Incremented it of projects
        LastID: ProjectId;

        /// Storage for carbon credits passports
        CarbonCreditPassportRegistry
            get(fn registry_by_asseid):
            map hasher(blake2_128_concat) AssetId<T> => Option<CarbonCreditsPassport<AssetId<T>>>;

        /// Storage for user burn sertificates
        BurnCertificates
            get(fn cert_by_account_id):
            map hasher(blake2_128_concat) T::AccountId => Vec<CarbonCreditsBurnCertificate<AssetId<T>, T::Balance>>;
    }
}

// Pallet events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        AssetId = <T as pallet_assets::Config>::AssetId,
        Balance = <T as pallet_assets::Config>::Balance,
    {
        // Project Events:

        /// \[ProjectOwner, ProjectId\]
        ProjectCreated(AccountId, ProjectId),
        /// \[ProjectOwner, ProjectId, OldStandard, NewStandard\]
        ProjectStandardChanged(AccountId, ProjectId, Standard, Standard),
        /// \[ProjectOwner, ProjectId\]
        ProjectSubmited(AccountId, ProjectId),
        /// \[Auditor, ProjectId\]
        ProjectSignedByAduitor(AccountId, ProjectId),
        /// \[StandardRoleAccount, ProjectId\]
        ProjectSignedByStandard(AccountId, ProjectId),
        /// \[Registry, ProjectId\]
        ProjectSignedByRegistry(AccountId, ProjectId),
        /// \[ProjectOwner, Signer, Role, ProjectId\]
        ProjectSignerAdded(AccountId, AccountId, RoleMask, ProjectId),
        /// \[ProjectOwner, Signer, Role, ProjectId\]
        ProjectSignerRemoved(AccountId, AccountId, RoleMask, ProjectId),

        // Annual Report Events:

        /// \[ProjectOwner, ProjectId\]
        AnnualReportCreated(AccountId, ProjectId),
        /// \[ProjectOwner, ProjectId\]
        AnnualReportDeleted(AccountId, ProjectId),
        // \[ProjectOwner, ProjectId, NewCount\]
        AnnualReportCreditsCountChanged(AccountId, ProjectId, Balance),
        /// \[ProjectOwner, ProjectId\]
        AnnualReportSubmited(AccountId, ProjectId), 
        /// \[Auditor, ProjectId\]
        AnnualReportSignedByAuditor(AccountId, ProjectId),
        /// \[StandardRoleAccount, ProjectId\]
        AnnualReportSignedByStandard(AccountId, ProjectId),
        /// \[Registry, ProjectId\]
        AnnualReportSignedByRegistry(AccountId, ProjectId),
        /// \[ProjectOwner, Signer, Role, ProjectId\]
        AnnualReportSignerAdded(AccountId, AccountId, RoleMask, ProjectId),
        /// \[ProjectOwner, Signer, Role, ProjectId\]
        AnnualReportSignerRemoved(AccountId, AccountId, RoleMask, ProjectId),

        // Carbon Credits Events:

        /// \[ProjectOwner, ProjectId, AssetId\]
        CarbonCreditsAssetCreated(AccountId, ProjectId, AssetId),
        /// \[ProjectOwner, AssetId\]
        CarbonCreditsMetadataChanged(AccountId, AssetId),
        /// \[ProjectOwner, ProjectId, AssetId\]
        CarbonCreditsMinted(AccountId, ProjectId, AssetId),
        /// \[CarbonCreditsHolder, AccountToTransfer, AssetId\]
        CarbonCreditsTransfered(AccountId, AccountId, AssetId),
        /// \[ProjectOwner, AssetId\]
        CarbonCreditsAssetBurned(AccountId, AssetId),
    }
);


decl_error! {
    pub enum Error for Module<T: Config> {

        // Project errors:

        /// Separate Error for project validation
        InvalidProjectState,

        // Account errors:

        /// Account does not have an auditor role in Accounts Pallet
        AccountNotAuditor,
        /// Account is not owner of the project or doenst have auditor role in Accounts Pallet
        AccountNotOwner,
        /// Account doesnt have Standard role in Accounts Pallet
        AccountNotStandard,
        /// Account doesnt have Registry role in Accounts Pallet 
        AccountNotRegistry,
        /// Account doesnt have Investor role in Accounts Pallet 
        AccountNotInvestor,
        /// Role if the account is incorrect
        AccountIncorrectRole,
        /// Account is not assigned as signer in given role
        AccountNotGivenRoleSigner,
        /// Account not owner of file
        AccountNotFileOwner,

        // State machine errors

        /// Invalid State of the state machine
        InvalidState,
        /// Project does not exits in the storage
        ProjectNotExist,
        /// Project doesnt have Registered state
        ProjectNotRegistered,
        /// Annual reports of the project do not exist
        NoAnnualReports,
        /// State of an annual report doesnt equal to Issued
        NotIssuedAnnualReportsExist,

        // Asset error

        /// Error has occured when thred to create asset
        ErrorCreatingAsset,
        /// Error minting asset
        ErrorMintingAsset,
        /// Carbon credits are already created error
        CCAlreadyCreated,
        /// Carbon credits transfer failed
        TransferFailed,
        /// Carbon Credits asset burn failed
        BurnFailed,
        /// Bad parameters of metadata
        BadMetadataParameters,
        /// Set metadata parameters failed
        SetMetadataFailed,
        /// Annual report is not ready
        AnnualReportNotReady,
        /// Carbon Credits Ballance too low
        InsufficientCarbonCredits,

        // Passport Errors:

        /// There is no carbon credits passport in storage
        PassportNotExist,
        /// Project referenced by passport is equal to given
        BadPassportProject,
        /// Given Annual report index is bad 
        BadPassportAnnualReport,

        // Signer errors:

        /// Signer does not exist in Project required signers
        IncorrectProjectSigner,
        /// Signer does not exist in annual report required signers
        IncorrectAnnualReportSigner,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// <pre>
        /// Method: create_project(standard: Standard, file_id: FileId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            standard: Standard - Carbon Credits Standard
        ///            file_id: FileId - id of file in filesign pallet
        /// Access: Project Owner Role
        ///
        /// Creates new project with relation to PDD file in filesign
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1, 2)]
        pub fn create_project(origin, standard: Standard, file_id: FileId) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ensure!(pallet_evercity_filesign::Module::<T>::address_is_owner_for_file(file_id, &caller), Error::<T>::AccountNotFileOwner);

            let new_id = LastID::get() + 1;
            let new_project = ProjectStruct::<<T as frame_system::Config>::AccountId, T, T::Balance>::new(caller.clone(), new_id, standard, file_id);
            <ProjectById<T>>::insert(new_id, new_project);
            LastID::mutate(|x| *x = x.checked_add(1).unwrap());

            // SendEvent
            Self::deposit_event(RawEvent::ProjectCreated(caller, new_id));
            Ok(())
        }

        /// <pre>
        /// Method: change_project_standard(project_id: ProjectId, standard: Standard)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId
        ///            standard: Standard - Carbon Credits Standard
        /// Access: Project Owner Role
        ///
        /// Creates new project with relation to PDD file in filesign
        /// </pre>
        // #[weight = 10_000 + T::DbWeight::get().reads_writes(1, 2)]
        // pub fn change_project_standard(origin, project_id: ProjectId, standard: Standard) -> DispatchResult {
        //     todo!("just one standard");
        //     let caller = ensure_signed(origin)?;
        //     ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);

        //     ProjectById::<T>::try_mutate(
        //         project_id, |project_to_mutate| -> DispatchResult {
        //             match project_to_mutate  {
        //                 None => return Err(Error::<T>::ProjectNotExist.into()),
        //                 Some(proj) => {
        //                     ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
        //                     ensure!(proj.state == project::PROJECT_OWNER_SIGN_PENDING, Error::<T>::InvalidProjectState);
        //                 }
        //             }

        //             Ok(())
        //      })?;

        //     // let new_id = LastID::get() + 1;
        //     // let new_project = ProjectStruct::<<T as frame_system::Config>::AccountId, T, T::Balance>::new(caller.clone(), new_id, standard, file_id);
        //     // <ProjectById<T>>::insert(new_id, new_project);
        //     // LastID::mutate(|x| *x = x.checked_add(1).unwrap());

        //     // SendEvent
        //     //ProjectStandardChanged
        //     // Self::deposit_event(RawEvent::ProjectCreated(caller, new_id));
        //     todo!();
        //     Ok(())
        // }

        /// <pre>
        /// Method: assign_project_signer(signer: T::AccountId, role: RoleMask, project_id: ProjectId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            signer: T::AccountId - assign signer account
        ///            role - Role of the signer
        ///            project_id - id of the project
        ///
        /// Access: Owner of the project 
        ///
        /// assign signer, that is required for signing project documentation
        /// also adds signer to filesign PDD 
        /// 
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1, 1)]
        pub fn assign_project_signer(origin, signer: T::AccountId, role: RoleMask, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            proj.assign_required_signer((signer.clone(), role));

                            pallet_evercity_filesign::Module::<T>::assign_signer(origin, proj.file_id, signer.clone())?;
                        }
                    }

                    Ok(())
             })?;
            Self::deposit_event(RawEvent::ProjectSignerAdded(caller, signer, role, project_id));
            Ok(())
        }

        /// <pre>
        /// Method: remove_project_signer(signer: T::AccountId, role: RoleMask, project_id: ProjectId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            signer: T::AccountId - assign signer account
        ///            role - Role of the signer
        ///            project_id - id of the project
        ///
        /// Access: Owner of the project 
        ///
        /// remove signer, that is was added for signing project documentation
        /// also deletes signer from filesign PDD 
        /// 
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1, 1)]
        pub fn remove_project_signer(origin, signer: T::AccountId, role: RoleMask, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(proj.is_required_signer((signer.clone(), role)), Error::<T>::AccountNotGivenRoleSigner);
                            proj.remove_required_signer((signer.clone(), role));
                            let hastable = std::collections::HashMap::<i32,i32>::new();

                            todo!("check if signed");
                            pallet_evercity_filesign::Module::<T>::delete_signer(origin, proj.file_id, signer.clone())?;
                        }
                    }

                    Ok(())
             })?;
            Self::deposit_event(RawEvent::ProjectSignerRemoved(caller, signer, role, project_id));
            Ok(())
        }

        /// <pre>
        /// Method: sign_project(project_id: ProjectId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id - id of the project
        ///
        /// Access: Required Signer with signer role 
        ///
        /// Signs project documentation, changing state of the project state machine
        /// 
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 2)]
        pub fn sign_project(origin, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            let mut event_opt: Option<Event<T>> = None;
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    ensure!(project_to_mutate.is_some(), Error::<T>::ProjectNotExist);
                    let project_documentation_file_id = project_to_mutate.as_ref().unwrap().file_id;
                    ensure!(pallet_evercity_filesign::Module::<T>::address_is_signer_for_file(project_documentation_file_id, &caller), 
                        Error::<T>::IncorrectProjectSigner);
                    Self::change_project_state(&mut project_to_mutate.as_mut().unwrap(), caller, &mut event_opt)?;
                    pallet_evercity_filesign::Module::<T>::sign_latest_version(origin, project_documentation_file_id)?;
                    Ok(())
             })?;
            if let Some(event) = event_opt {
                Self::deposit_event(event);
            }
            Ok(())
        }

        /// <pre>
        /// Method: create_annual_report(project_id: ProjectId, file_id: FileId, carbon_credits_count: T::Balance)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId - Id of project, where to create annual report
        ///            file_id: FileId - Id of pre created file of annual report document
        ///            carbon_credits_count - count of carbon credits to release after signing
        ///
        ///
        /// Access: Owner of the project
        ///
        /// Create annual report entity with link to annual report file
        /// 
        /// </pre> 
        #[weight = 10_000 + T::DbWeight::get().reads_writes(3, 1)]
        pub fn create_annual_report(
            origin, 
            project_id: ProjectId, 
            file_id: FileId, 
            carbon_credits_count: T::Balance,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ensure!(pallet_evercity_filesign::Module::<T>::address_is_owner_for_file(file_id, &caller), Error::<T>::AccountNotFileOwner);
            ProjectById::<T>::try_mutate(
                project_id, |project_option| -> DispatchResult {
                    match project_option {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(project.state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
                            ensure!(project.annual_reports.iter()
                                        .all(|x| x.state == annual_report::REPORT_ISSUED),
                                Error::<T>::NotIssuedAnnualReportsExist
                            );
                            let meta = annual_report::CarbonCreditsMeta::new(name, symbol, decimals);
                            ensure!(meta.is_metadata_valid(), Error::<T>::BadMetadataParameters);
                            project.annual_reports
                                        .push(annual_report::AnnualReportStruct::<T::AccountId, T, T::Balance>::new(file_id, carbon_credits_count, Timestamp::<T>::get(), meta));
                            Ok(())
                        }
                    }
             })?;
            // SendEvent
            Self::deposit_event(RawEvent::AnnualReportCreated(caller, project_id));
            Ok(())
        }

        /// <pre>
        /// Method: change_report_carbon_credits_count(project_id: ProjectId, new_carbon_credits_count: T::Balance)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId - Id of project, where to create annual report
        ///            new_carbon_credits_count - new count of carbon credits to release after signing
        ///
        ///
        /// Access: Owner of the project
        ///
        /// Change annual report balance. Can only be changed in preparing step
        /// 
        /// </pre> 
        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        pub fn change_report_carbon_credits_count(origin, project_id: ProjectId, new_carbon_credits_count: T::Balance) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            // Ensure, that report in initial preparing state REPORT_PROJECT_OWNER_SIGN_PENDING
                            ensure!(proj.annual_reports[len - 1].state == annual_report::REPORT_PROJECT_OWNER_SIGN_PENDING, Error::<T>::InvalidState);
                            proj.annual_reports[len - 1].change_carbon_credits_count(new_carbon_credits_count);
                        }
                    }
                    Ok(())
             })?;
            // SendEvent
            Self::deposit_event(RawEvent::AnnualReportCreditsCountChanged(caller, project_id, new_carbon_credits_count));
            Ok(())
        }


        /// <pre>
        /// Method: delete_last_annual_report(project_id: ProjectId: T::Balance)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId - Id of project, where to create annual report
        ///
        ///
        /// Access: Owner of the project
        ///
        /// Deletes project's last annual report if it is not issued
        /// 
        /// </pre> 
        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        pub fn delete_last_annual_report(origin, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            // Ensure, that report not in final issued state
                            // To prevent deleting ready reports, that have its carbon credits released
                            ensure!(proj.annual_reports[len - 1].state != annual_report::REPORT_ISSUED, Error::<T>::InvalidState);
                            proj.annual_reports.remove(len - 1);
                        }
                    }
                    Ok(())
             })?;
            // SendEvent
            Self::deposit_event(RawEvent::AnnualReportDeleted(caller, project_id));
            Ok(())
        }


        /// <pre>
        /// Method: assign_last_annual_report_signer(signer: T::AccountId, role: RoleMask, project_id: ProjectId))
        /// Arguments: origin: AccountId - Transaction caller
        ///            signer: T::AccountId - assign signer account
        ///            role - Role of the signer
        ///            project_id - id of the project
        ///
        /// Access: Owner of the project
        /// assign signer, that is required for signing фттгфд кузщке document
        /// also adds signer to filesign document 
        /// 
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(3, 2)]
        pub fn assign_last_annual_report_signer(origin, signer: T::AccountId, role: RoleMask, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            proj.annual_reports[len - 1].assign_required_signer((signer.clone(), role));
                            // Assign signer in filesign pallet:
                            pallet_evercity_filesign::Module::<T>::assign_signer(origin.clone(), proj.annual_reports[len - 1].file_id, signer.clone())?;
                        }
                    }
                    Ok(())
             })?;
            Self::deposit_event(RawEvent::AnnualReportSignerAdded(caller, signer, role, project_id));
            Ok(())
        }

        /// <pre>
        /// Method: remove_last_annual_report_signer(signer: T::AccountId, role: RoleMask, project_id: ProjectId))
        /// Arguments: origin: AccountId - Transaction caller
        ///            signer: T::AccountId - assign signer account
        ///            role - Role of the signer
        ///            project_id - id of the project
        ///
        /// Access: Owner of the project
        /// remove signer, that was added for signing фттгфд кузщке document
        /// also deletes signer to filesign document 
        /// 
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 2)]
        pub fn remove_last_annual_report_signer(origin, signer: T::AccountId, role: RoleMask, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            proj.annual_reports[len - 1].remove_required_signer((signer.clone(), role));
                            // delete signer in filesign pallet
                            todo!("check if signed");
                            pallet_evercity_filesign::Module::<T>::delete_signer(origin.clone(), proj.annual_reports[len - 1].file_id, signer.clone())?;
                        }
                    }
                    Ok(())
             })?;
            Self::deposit_event(RawEvent::AnnualReportSignerRemoved(caller, signer, role, project_id));
            Ok(())
        }

        /// <pre>
        /// Method: sign_last_annual_report(project_id: ProjectId)
        /// Arguments: origin: AccountId - Transaction caller
        ///
        /// Access: Assigned signer
        ///
        /// Signs annual repor document, changing state of the project state machine
        /// 
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(3, 2)]
        pub fn sign_last_annual_report(origin, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            let mut event_opt: Option<Event<T>> = None;
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            let len = project.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            let annual_report_file_id =  project.annual_reports[len - 1].file_id;
                            ensure!(pallet_evercity_filesign::Module::<T>::address_is_signer_for_file(annual_report_file_id, &caller), 
                                Error::<T>::IncorrectAnnualReportSigner);
                            Self::change_project_annual_report_state(project, caller, &mut event_opt)?;
                            pallet_evercity_filesign::Module::<T>::sign_latest_version(origin, 
                                annual_report_file_id)?;
                        }
                    }
                    Ok(())
            })?;
            if let Some(event) = event_opt {
                Self::deposit_event(event);
            }
            Ok(())
        }

        /// <pre>
        /// Method: set_carbon_credits_metadata(
        ///             asset_id: <T as pallet_assets::Config>::AssetId,
        ///             name: Vec<u8>,
        ///             symbol: Vec<u8>,
        ///             decimals: u8,
        ///             )
        /// 
        /// Arguments: origin: AccountId - Transaction caller
        ///            asset_id - id of asset in assets pallet
        ///            name - asset name
        ///            symbol - asset symbol
        ///            decimals - decimals count
        ///
        /// Access: Asset Owner
        ///
        /// Sets CC asset metadata
        /// 
        /// </pre>
        // #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 2)]
        // pub fn set_carbon_credits_metadata(
        //     origin, 
        //     asset_id: <T as pallet_assets::Config>::AssetId,
        //     name: Vec<u8>,
        //     symbol: Vec<u8>,
        //     decimals: u8,
        // ) -> DispatchResult {
        //     todo!("change function signatchure");
        //     let owner = ensure_signed(origin.clone())?;
            
        //     // check passport creds
        //     let passport = CarbonCreditPassportRegistry::<T>::get(asset_id);
        //     ensure!(passport.is_some(), Error::<T>::PassportNotExist);

        //     ensure!(!name.is_empty() && !symbol.is_empty(), Error::<T>::BadMetadataParameters);
        //     let transfer_call = pallet_assets::Call::<T>::set_metadata(asset_id, name, symbol, decimals);
        //     let result = transfer_call.dispatch_bypass_filter(origin);
        //     ensure!(!result.is_err(), Error::<T>::SetMetadataFailed);

        //     Self::deposit_event(RawEvent::CarbonCreditsMetadataChanged(owner, asset_id));
        //     Ok(())
        // }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(5, 4)]
        pub fn release_carbon_credits(
            origin, 
            project_id: ProjectId,
            asset_id: <T as pallet_assets::Config>::AssetId,
            new_carbon_credits_holder: T::AccountId,
            min_balance: <T as pallet_assets::Config>::Balance,
        ) -> DispatchResult {
            let project_owner = ensure_signed(origin.clone())?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&project_owner), Error::<T>::AccountNotOwner);

            ProjectById::<T>::try_mutate(
                project_id, |project_option| -> DispatchResult {
                    match project_option {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == project_owner, Error::<T>::AccountNotOwner);
                            ensure!(project.state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
        
                            // Check that there is at least one annual report
                            let reports_len = project.annual_reports.len();
                            ensure!(reports_len > 0,
                                Error::<T>::NoAnnualReports
                            );
        
                            // ensure that carbon credits not released, and then set it to released state
                            let last_annual_report = &mut project.annual_reports[reports_len - 1];
                            ensure!(!last_annual_report.is_carbon_credits_released(), Error::<T>::CCAlreadyCreated);
                            last_annual_report.set_carbon_credits_released();
            
                            // Create Asset:
                            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(new_carbon_credits_holder);
                            let create_asset_call = pallet_assets::Call::<T>::create(asset_id, new_carbon_credits_holder_source, 0, min_balance);
                            let create_asset_result = create_asset_call.dispatch_bypass_filter(origin.clone());
                            ensure!(!create_asset_result.is_err(), Error::<T>::ErrorCreatingAsset);
        
          
                            // Set metadata from annual report
                            // Changing metadata of annual report to empty struct
                            let mut meta = annual_report::CarbonCreditsMeta::default();
                            sp_std::mem::swap(&mut meta, &mut last_annual_report.carbon_credits_meta);
                            let set_metadata_call = pallet_assets::Call::<T>::set_metadata(asset_id, 
                                                    meta.name, 
                                                    meta.symbol, 
                                                    meta.decimals
                                                );
        
                            let result = set_metadata_call.dispatch_bypass_filter(origin.clone());
                            ensure!(!result.is_err(), {
                                // destroy if failed
                                pallet_assets::Call::<T>::destroy(asset_id, 0);
                                Error::<T>::SetMetadataFailed
                            });
        
                            // Mint Carbon Credits
                            let cc_amount = last_annual_report.carbon_credits_count();
                            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(project_owner.clone());
                            let mint_call = pallet_assets::Call::<T>::mint(asset_id, new_carbon_credits_holder_source, cc_amount);
                            let result = mint_call.dispatch_bypass_filter(origin);
                            ensure!(!result.is_err(), {
                                // destroy if failed
                                pallet_assets::Call::<T>::destroy(asset_id, 0);
                                Error::<T>::ErrorMintingAsset
                            });
        
                            // Create passport
                            <CarbonCreditPassportRegistry<T>>::insert(asset_id, CarbonCreditsPassport::new(asset_id, project_id, project.annual_reports.len()));
                            Ok(())
                        }
                    }
             })?;
    
            Self::deposit_event(RawEvent::CarbonCreditsMinted(project_owner, project_id, asset_id));
            Ok(())
        }

        /// <pre>
        /// Method: transfer_carbon_credits(
        ///    asset_id: <T as pallet_assets::Config>::AssetId, 
        ///    new_carbon_credits_holder: T::AccountId, 
        ///    amount: T::Balance
        ///) 
        /// Arguments: origin: AccountId - Transaction caller
        ///
        /// Access: Carbon Credits holder
        ///
        ///  Transfers carbon creadits of asset id in given amount to an adress
        /// 
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        pub fn transfer_carbon_credits(
            origin, 
            asset_id: <T as pallet_assets::Config>::AssetId, 
            new_carbon_credits_holder: T::AccountId, 
            amount: T::Balance
        ) -> DispatchResult {
            let owner = ensure_signed(origin.clone())?;
            // check passport creds
            let passport = CarbonCreditPassportRegistry::<T>::get(asset_id);
            ensure!(passport.is_some(), Error::<T>::PassportNotExist);

            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(new_carbon_credits_holder.clone().into());
            let transfer_call = pallet_assets::Call::<T>::transfer(asset_id, new_carbon_credits_holder_source, amount);
            let result = transfer_call.dispatch_bypass_filter(origin);
            ensure!(!result.is_err(), Error::<T>::TransferFailed);

            Self::deposit_event(RawEvent::CarbonCreditsTransfered(owner, new_carbon_credits_holder, asset_id));
            Ok(())
        }

        /// <pre>
        /// Method: burn_carbon_credits(
        ///    asset_id: <T as pallet_assets::Config>::AssetId, 
        ///    amount: T::Balance
        ///) 
        /// Arguments: origin: AccountId - Transaction caller
        ///            asset_id - id of asset_id
        ///            amount - amount to burn
        ///
        /// Access: Holder of carbon credits
        ///
        /// Burns amount of carbon credits
        /// 
        /// </pre>
        #[weight = 10_000 + T::DbWeight::get().reads_writes(3, 2)]
        pub fn burn_carbon_credits(
            origin, 
            asset_id: <T as pallet_assets::Config>::AssetId, 
            amount: T::Balance
        ) -> DispatchResult {
            let credits_holder = ensure_signed(origin.clone())?;
            // check passport creds
            let passport = CarbonCreditPassportRegistry::<T>::get(asset_id);
            ensure!(passport.is_some(), Error::<T>::PassportNotExist);
            ensure!(pallet_assets::Pallet::<T>::balance(asset_id, credits_holder.clone()) >= amount,
                Error::<T>::InsufficientCarbonCredits
            );

            BurnCertificates::<T>::try_mutate(
                credits_holder.clone(), |certificates| -> DispatchResult {
                    match certificates.iter_mut().find(|x| x.asset_id == asset_id) {
                        Some(cert) => {
                            cert.burn_amount += amount;
                        },
                        None => {
                            certificates.push(CarbonCreditsBurnCertificate::new(asset_id, amount));
                        }
                    }

                    let burn_call = pallet_assets::Call::<T>::burn_self_assets(asset_id, amount);
                    let result = burn_call.dispatch_bypass_filter(origin);
                    ensure!(!result.is_err(), Error::<T>::BurnFailed);
                    Ok(())
                }

            )?;

            Self::deposit_event(RawEvent::CarbonCreditsAssetBurned(credits_holder, asset_id));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    /// Changes state of a project by signing
    fn change_project_state(project: &mut ProjectStruct<T::AccountId, T, T::Balance>, caller: T::AccountId, event: &mut Option<Event<T>>) -> DispatchResult {
        match &mut project.get_standard() {
            // Project Owner submits PDD (changing status to Registration) => 
            // => Auditor Approves PDD => Standard Certifies PDD => Registry Registers PDD (changing status to Issuance)
            Standard::GOLD_STANDARD  => {
                match project.state {
                    project::PROJECT_OWNER_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                        ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                        ensure!(Self::is_correct_project_signer(&project, caller.clone(), accounts::accounts::CC_PROJECT_OWNER_ROLE_MASK), 
                            Error::<T>::IncorrectProjectSigner);
                        project.state = project::AUDITOR_SIGN_PENDING;
                        project.status = project::ProjectStatus::REGISTRATION;
                        *event = Some(RawEvent::ProjectSubmited(caller, project.id));
                    },
                    project::AUDITOR_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                        ensure!(Self::is_correct_project_signer(&project, caller.clone(), accounts::accounts::CC_AUDITOR_ROLE_MASK), 
                            Error::<T>::IncorrectProjectSigner);
                        project.state = project::STANDARD_SIGN_PENDING;
                        *event = Some(RawEvent::ProjectSignedByAduitor(caller, project.id));
                    },
                    project::STANDARD_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                        ensure!(Self::is_correct_project_signer(&project, caller.clone(), accounts::accounts::CC_STANDARD_ROLE_MASK), 
                            Error::<T>::IncorrectProjectSigner);
                        project.state = project::REGISTRY_SIGN_PENDING;
                        *event = Some(RawEvent::ProjectSignedByStandard(caller, project.id));
                    },
                    project::REGISTRY_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                        ensure!(Self::is_correct_project_signer(&project, caller.clone(), accounts::accounts::CC_REGISTRY_ROLE_MASK), 
                            Error::<T>::IncorrectProjectSigner);
                        project.state = project::REGISTERED;
                        project.status = project::ProjectStatus::ISSUANCE;
                        *event = Some(RawEvent::ProjectSignedByRegistry(caller, project.id));
                    },
                    _ => return Err(Error::<T>::InvalidState.into())
                }
                Ok(())
            }
        }
    }

    /// Changes state of an annual report by signing
    fn change_project_annual_report_state(project: &mut ProjectStruct<T::AccountId, T, T::Balance>, caller: T::AccountId, event: &mut Option<Event<T>>) -> DispatchResult {
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
                        ensure!(Self::is_correct_annual_report_signer(&report, caller.clone(), accounts::accounts::CC_PROJECT_OWNER_ROLE_MASK),
                            Error::<T>::IncorrectProjectSigner);
                        ensure!(report.carbon_credits_meta.is_metadata_valid(), Error::<T>::BadMetadataParameters);
                        report.state = annual_report::REPORT_AUDITOR_SIGN_PENDING;
                        *event = Some(RawEvent::AnnualReportSubmited(caller, project.id));
                    },
                    annual_report::REPORT_AUDITOR_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                        ensure!(Self::is_correct_annual_report_signer(&report, caller.clone(), accounts::accounts::CC_AUDITOR_ROLE_MASK),
                            Error::<T>::IncorrectProjectSigner);
                        report.state = annual_report::REPORT_STANDARD_SIGN_PENDING;
                        *event = Some(RawEvent::AnnualReportSignedByAuditor(caller, project.id));
                    },
                    annual_report::REPORT_STANDARD_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                        ensure!(Self::is_correct_annual_report_signer(&report, caller.clone(), accounts::accounts::CC_STANDARD_ROLE_MASK),
                            Error::<T>::IncorrectProjectSigner);
                        report.state = annual_report::REPORT_REGISTRY_SIGN_PENDING;
                        *event = Some(RawEvent::AnnualReportSignedByStandard(caller, project.id));
                    },
                    annual_report::REPORT_REGISTRY_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                        ensure!(Self::is_correct_annual_report_signer(&report, caller.clone(), accounts::accounts::CC_REGISTRY_ROLE_MASK),
                            Error::<T>::IncorrectProjectSigner);
                        report.state = annual_report::REPORT_ISSUED;
                        *event = Some(RawEvent::AnnualReportSignedByRegistry(caller, project.id));
                    },
                    _ => return Err(Error::<T>::InvalidState.into())
                }
                Ok(())
            },
        }
    }

    fn is_correct_project_signer(project: &ProjectStruct<T::AccountId, T, T::Balance>, account: T::AccountId, role: RoleMask) -> bool {
        pallet_evercity_accounts::Module::<T>::account_is_selected_role(&account, role) &&
        project.is_required_signer((account, role))
    }

    fn is_correct_annual_report_signer(annual_report: &annual_report::AnnualReportStruct<T::AccountId, T, T::Balance>, account: T::AccountId, role: RoleMask) -> bool {
        pallet_evercity_accounts::Module::<T>::account_is_selected_role(&account, role) &&
        annual_report.is_required_signer((account, role))
    }

    #[cfg(test)]
    pub fn get_proj_by_id(id: ProjectId) -> Option<ProjectStruct<T::AccountId, T, T::Balance>> {
        ProjectById::<T>::get(id)
    }

    #[cfg(test)]
    pub fn get_passport_by_assetid(asset_id: AssetId<T>) -> Option<CarbonCreditsPassport<AssetId<T>>> {
        CarbonCreditPassportRegistry::<T>::get(asset_id)
    }

    #[cfg(test)]
    pub fn get_certificates_by_account(account: T::AccountId) -> Vec<CarbonCreditsBurnCertificate<AssetId<T>, T::Balance>> {
        BurnCertificates::<T>::get(account)
    }
}