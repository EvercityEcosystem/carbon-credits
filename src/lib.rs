#![cfg_attr(not(feature = "std"), no_std)]
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
pub use pallet_assets::weights::WeightInfo;
use pallet_evercity_accounts as accounts;
use project::{ProjectStruct, ProjectId};
use standard::Standard;
use pallet_evercity_filesign::{FileId};
use pallet_evercity_accounts::accounts::RoleMask;
use carbon_credits_passport::CarbonCreditsPassport;

pub mod standard;
pub mod project;
pub mod annual_report;
pub mod file_hash;
pub mod required_signers;
pub mod carbon_credits_passport;

#[cfg(test)]    
pub mod tests;

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

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        ProjectById
            get(fn project_by_id):
            map hasher(blake2_128_concat) u32 => Option<ProjectStruct<T::AccountId, T, T::Balance>>;

        LastID: ProjectId;

        CarbonCreditPassportRegistry
            get(fn registry_by_asseid):
            map hasher(blake2_128_concat) AssetId<T> => Option<CarbonCreditsPassport<AssetId<T>>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        AssetId = <T as pallet_assets::Config>::AssetId
    {
        // Project Events:
        ProjectCreated(AccountId, ProjectId),
        ProjectSubmited(AccountId, ProjectId),    
        ProjectRegistered(AccountId, ProjectId),
        ProjectSignedByAduitor(AccountId, ProjectId),
        ProjectSignedByStandard(AccountId, ProjectId),
        ProjectSignedByRegistry(AccountId, ProjectId),
        ProjectSignerAdded(AccountId, AccountId, RoleMask, ProjectId),

        // Annual Report Events:
        AnnualReportCreated(AccountId, ProjectId),
        AnnualReportSubmited(AccountId, ProjectId),
        AnnualReportSignedByAuditor(AccountId, ProjectId),
        AnnualReportSignedByStandard(AccountId, ProjectId),
        AnnualReportSignedByRegistry(AccountId, ProjectId),
        AnnualReportSignerAdded(AccountId, AccountId, RoleMask, ProjectId),

        // Carbon Credits Events:
        CarbonCreditsAssetCreated(AccountId, ProjectId, AssetId),
        CarbonCreditsMetadataChanged(AccountId, AssetId),
        CarbonCreditsMinted(AccountId, ProjectId, AssetId),
        CarbonCreditsTransfered(AccountId, AccountId, AssetId),
        CarbonCreditsAssetBurned(AccountId, AssetId),
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
        AccountIncorrectRole,

        InvalidAction,
        InvalidState,
        InvalidStandard,
        ProjectNotExist,
        ProjectNotRegistered,
        NoAnnualReports,
        NotIssuedAnnualReportsExist,

        ErrorCreatingAsset,
        ErrorMintingAsset,
        CCAlreadyCreated,
        TransferFailed,
        BurnFailed,
        BadMetadataParameters,
        SetMetadataFailed,
        AnnualReportNotReady,

        // Passport Errors
        PassportNotExist,
        BadPassportProject,
        BadPassportAnnualReport,

        // Sign errors
        IncorrectProjectSigner,
        IncorrectAnnualReportSigner,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn create_project(origin, standard: Standard, file_id: FileId) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ensure!(pallet_evercity_filesign::Module::<T>::address_is_owner_for_file(file_id, &caller), Error::<T>::AccountNotOwner);

            let new_id = LastID::get() + 1;
            let new_project = ProjectStruct::<<T as frame_system::Config>::AccountId, T, T::Balance>::new(caller.clone(), new_id, standard, file_id);
            <ProjectById<T>>::insert(new_id, new_project);
            LastID::mutate(|x| *x = x.checked_add(1).unwrap());

            // SendEvent
            Self::deposit_event(RawEvent::ProjectCreated(caller, new_id));
            Ok(())
        }

        #[weight = 10_000]
        pub fn assign_project_signer(origin, signer: T::AccountId, role: RoleMask, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist)?,
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

        #[weight = 10_000]
        pub fn sign_project(origin, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            let mut event_opt: Option<Event<T>> = None;
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    ensure!(project_to_mutate.is_some(), Error::<T>::ProjectNotExist);
                    pallet_evercity_filesign::Module::<T>::sign_latest_version(origin, project_to_mutate.as_ref().unwrap().file_id)?;
                    Self::change_project_state(&mut project_to_mutate.as_mut().unwrap(), caller, &mut event_opt)?;
                    Ok(())
             })?;
            if let Some(event) = event_opt {
                Self::deposit_event(event);
            }
            Ok(())
        }

        #[weight = 10_000]
        pub fn create_annual_report(origin, project_id: ProjectId, file_id: FileId, carbon_credits_count: T::Balance) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ensure!(pallet_evercity_filesign::Module::<T>::address_is_owner_for_file(file_id, &caller), Error::<T>::AccountNotOwner);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    ensure!(project_to_mutate.is_some(), Error::<T>::ProjectNotExist);
                    ensure!(project_to_mutate.as_ref().unwrap().owner == caller, Error::<T>::AccountNotOwner);
                    ensure!(project_to_mutate.as_ref().unwrap().state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
                    ensure!(project_to_mutate.as_ref().unwrap().annual_reports.iter()
                                .all(|x| x.state == annual_report::REPORT_ISSUED),
                        Error::<T>::NotIssuedAnnualReportsExist
                    );
                    project_to_mutate.as_mut().unwrap().annual_reports
                                .push(annual_report::AnnualReportStruct::<T::AccountId, T, T::Balance>::new(file_id, carbon_credits_count, Timestamp::<T>::get()));
                    Ok(())
             })?;
            // SendEvent
            Self::deposit_event(RawEvent::AnnualReportCreated(caller, project_id));
            Ok(())
        }

        #[weight = 10_000]
        pub fn assign_last_annual_report_signer(origin, signer: T::AccountId, role: RoleMask, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist)?,
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            proj.annual_reports[len - 1].assign_required_signer((signer.clone(), role));
                            pallet_evercity_filesign::Module::<T>::assign_signer(origin.clone(), proj.annual_reports[len - 1].file_id, signer.clone())?;
                        }
                    }
                    Ok(())
             })?;
            Self::deposit_event(RawEvent::AnnualReportSignerAdded(caller, signer, role, project_id));
            Ok(())
        }

        #[weight = 10_000]
        pub fn sign_last_annual_report(origin, project_id: ProjectId) -> DispatchResult {
            let caller = ensure_signed(origin.clone())?;
            let mut event_opt: Option<Event<T>> = None;
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    ensure!(project_to_mutate.is_some(), Error::<T>::ProjectNotExist);
                    ensure!(project_to_mutate.as_ref().unwrap().annual_reports.last().is_some(), Error::<T>::NoAnnualReports);
                    pallet_evercity_filesign::Module::<T>::sign_latest_version(origin, project_to_mutate.as_ref().unwrap().file_id)?;
                    Self::change_project_annual_report_state(&mut project_to_mutate.as_mut().unwrap(), caller, &mut event_opt)?;
                    Ok(())
            })?;
            if let Some(event) = event_opt {
                Self::deposit_event(event);
            }
            Ok(())
        }

        #[weight = 10_000]
        pub fn create_carbon_credits(
            origin, 
            asset_id: <T as pallet_assets::Config>::AssetId, 
            new_carbon_credits_holder: T::AccountId,
            min_balance: <T as pallet_assets::Config>::Balance,
            project_id: ProjectId,
        ) -> DispatchResult {
            let project_owner = ensure_signed(origin.clone())?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&project_owner), Error::<T>::AccountNotOwner);
    
            let project = ProjectById::<T>::get(project_id);
            ensure!(project.is_some(), Error::<T>::ProjectNotExist);
            ensure!(project.as_ref().unwrap().owner == project_owner, Error::<T>::AccountNotOwner);
            ensure!(project.as_ref().unwrap().state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
            // Annual Report Check:
            ensure!(project.as_ref().unwrap().annual_reports.last().is_some(), Error::<T>::NoAnnualReports);
            ensure!(project.as_ref().unwrap().annual_reports.last().unwrap().is_full_signed(), Error::<T>::AnnualReportNotReady);
    
            // Create Asset:
            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(new_carbon_credits_holder.into());
            let create_call = pallet_assets::Call::<T>::create(asset_id, new_carbon_credits_holder_source, 0, min_balance);
            let result = create_call.dispatch_bypass_filter(origin);
            ensure!(!result.is_err(), Error::<T>::ErrorCreatingAsset);

            // Create passport
            <CarbonCreditPassportRegistry<T>>::insert(asset_id, CarbonCreditsPassport::new(asset_id, project_id, project.as_ref().unwrap().annual_reports.len()));

            Self::deposit_event(RawEvent::CarbonCreditsAssetCreated(project_owner, project_id, asset_id));
            Ok(())
        }

        #[weight = 10_000]
        pub fn set_carbon_credits_metadata(
            origin, 
            asset_id: <T as pallet_assets::Config>::AssetId, 
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> DispatchResult {
            let owner = ensure_signed(origin.clone())?;
            // check passport creds
            let passport = CarbonCreditPassportRegistry::<T>::get(asset_id);
            ensure!(passport.is_some(), Error::<T>::PassportNotExist);

            ensure!(name.len() != 0 && symbol.len() != 0, Error::<T>::BadMetadataParameters);
            let transfer_call = pallet_assets::Call::<T>::set_metadata(asset_id, name, symbol, decimals);
            let result = transfer_call.dispatch_bypass_filter(origin);
            ensure!(!result.is_err(), Error::<T>::SetMetadataFailed);

            Self::deposit_event(RawEvent::CarbonCreditsMetadataChanged(owner, asset_id));
            Ok(())
        }

        #[weight = 10_000]
        pub fn mint_carbon_credits(origin, asset_id: <T as pallet_assets::Config>::AssetId, project_id: ProjectId) -> DispatchResult {
            let project_owner = ensure_signed(origin.clone())?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&project_owner), Error::<T>::AccountNotOwner);

            let mut cc_amount: Option<T::Balance> = None;
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    ensure!(project_to_mutate.is_some(), Error::<T>::ProjectNotExist);
                    ensure!(project_to_mutate.as_ref().unwrap().owner == project_owner, Error::<T>::AccountNotOwner);
                    ensure!(project_to_mutate.as_ref().unwrap().state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
    
                    // Check that there is at least one annual report
                    let reports_len = project_to_mutate.as_ref().unwrap().annual_reports.len();
                    ensure!(reports_len > 0,
                        Error::<T>::NoAnnualReports
                    );

                    // check passport creds
                    let passport = CarbonCreditPassportRegistry::<T>::get(asset_id);
                    ensure!(passport.is_some(), Error::<T>::PassportNotExist);
                    ensure!(passport.as_ref().unwrap().get_project_id() == project_id, Error::<T>::BadPassportProject);
                    ensure!(passport.as_ref().unwrap().get_last_report_index() == reports_len, Error::<T>::BadPassportAnnualReport);
    
                    // ensure that carbon credits not released, then
                    let last_annual_report = &mut project_to_mutate.as_mut().unwrap().annual_reports[reports_len - 1];
                    ensure!(!last_annual_report.is_carbon_credits_released(), Error::<T>::CCAlreadyCreated);
                    last_annual_report.set_carbon_credits_released();
    
                    cc_amount = Some(last_annual_report.carbon_credits_count());
                    Ok(())
             })?;
    
            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(project_owner.clone().into());
            let mint_call = pallet_assets::Call::<T>::mint(asset_id, new_carbon_credits_holder_source, cc_amount.unwrap());
            let result = mint_call.dispatch_bypass_filter(origin);
            ensure!(!result.is_err(), Error::<T>::ErrorMintingAsset);

            Self::deposit_event(RawEvent::CarbonCreditsMinted(project_owner, project_id, asset_id));
            Ok(())
        }

        #[weight = 10_000]
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

        #[weight = 10_000]
        pub fn burn_carbon_credits(
            origin, 
            asset_id: <T as pallet_assets::Config>::AssetId, 
            amount: T::Balance
        ) -> DispatchResult {
            let credits_holder = ensure_signed(origin.clone())?;
            // check passport creds
            let passport = CarbonCreditPassportRegistry::<T>::get(asset_id);
            ensure!(passport.is_some(), Error::<T>::PassportNotExist);

            let carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(credits_holder.clone().into());
            let burn_call = pallet_assets::Call::<T>::burn(asset_id, carbon_credits_holder_source, amount);
            let result = burn_call.dispatch_bypass_filter(origin);
            ensure!(!result.is_err(), Error::<T>::BurnFailed);

            Self::deposit_event(RawEvent::CarbonCreditsAssetBurned(credits_holder, asset_id));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
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
                        // project.signatures.push(caller.clone());
                        *event = Some(RawEvent::ProjectSubmited(caller, project.id));
                    },
                    project::AUDITOR_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                        ensure!(Self::is_correct_project_signer(&project, caller.clone(), accounts::accounts::CC_AUDITOR_ROLE_MASK), 
                            Error::<T>::IncorrectProjectSigner);
                        project.state = project::STANDARD_SIGN_PENDING;
                        // project.signatures.push(caller.clone());
                        *event = Some(RawEvent::ProjectSignedByAduitor(caller, project.id));
                    },
                    project::STANDARD_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                        ensure!(Self::is_correct_project_signer(&project, caller.clone(), accounts::accounts::CC_STANDARD_ROLE_MASK), 
                            Error::<T>::IncorrectProjectSigner);
                        project.state = project::REGISTRY_SIGN_PENDING;
                        // project.signatures.push(caller.clone());
                        *event = Some(RawEvent::ProjectSignedByStandard(caller, project.id));
                    },
                    project::REGISTRY_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                        ensure!(Self::is_correct_project_signer(&project, caller.clone(), accounts::accounts::CC_REGISTRY_ROLE_MASK), 
                            Error::<T>::IncorrectProjectSigner);
                        project.state = project::REGISTERED;
                        project.status = project::ProjectStatus::ISSUANCE;
                        // project.signatures.push(caller.clone());
                        *event = Some(RawEvent::ProjectSignedByRegistry(caller, project.id));
                    },
                    _ => Err(Error::<T>::InvalidState)?
                }
                Ok(())
            }
        }
    }

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
                        report.state = annual_report::REPORT_AUDITOR_SIGN_PENDING;

                        // report.signatures.push(caller.clone());
                        *event = Some(RawEvent::AnnualReportSubmited(caller, project.id));
                    },
                    annual_report::REPORT_AUDITOR_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                        ensure!(Self::is_correct_annual_report_signer(&report, caller.clone(), accounts::accounts::CC_AUDITOR_ROLE_MASK),
                            Error::<T>::IncorrectProjectSigner);
                        report.state = annual_report::REPORT_STANDARD_SIGN_PENDING;
                        // report.signatures.push(caller.clone());
                        *event = Some(RawEvent::AnnualReportSignedByAuditor(caller, project.id));
                    },
                    annual_report::REPORT_STANDARD_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                        ensure!(Self::is_correct_annual_report_signer(&report, caller.clone(), accounts::accounts::CC_STANDARD_ROLE_MASK),
                            Error::<T>::IncorrectProjectSigner);
                        report.state = annual_report::REPORT_REGISTRY_SIGN_PENDING;
                        // report.signatures.push(caller.clone());
                        *event = Some(RawEvent::AnnualReportSignedByStandard(caller, project.id));
                    },
                    annual_report::REPORT_REGISTRY_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                        ensure!(Self::is_correct_annual_report_signer(&report, caller.clone(), accounts::accounts::CC_REGISTRY_ROLE_MASK),
                            Error::<T>::IncorrectProjectSigner);
                        report.state = annual_report::REPORT_ISSUED;
                        // report.signatures.push(caller.clone());
                        report.set_full_signed();
                        *event = Some(RawEvent::AnnualReportSignedByRegistry(caller, project.id));
                    },
                    _ => Err(Error::<T>::InvalidState)?
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
}