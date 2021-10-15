#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
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
use project::{ProjectStruct, H256};
use standard::Standard;

pub mod standard;
pub mod project;
pub mod annual_report;

#[cfg(test)]    
pub mod tests;

pub trait Config: frame_system::Config + pallet_evercity_accounts::Config {}

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        ProjectById
            get(fn project_by_id):
            map hasher(blake2_128_concat) u32 => Option<ProjectStruct<T::AccountId>>;

        LastID: u32;
    }
}

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
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        pub fn create_project(origin, standard: Standard, filehash: H256) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::create_pdd(caller, standard, &filehash)?;
            Ok(())
        }

        #[weight = 10_000]
        pub fn create_annual_report(origin, project_id: u32, filehash: H256) -> DispatchResult {
            let caller = ensure_signed(origin)?;
           
            todo!();
            // Ok(())
        }

        #[weight = 10_000]
        pub fn sign_project(origin, proj_id: u32) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::sign_pdd(caller, proj_id)?;
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    pub fn create_pdd(caller: T::AccountId, standard: Standard, filehash: &H256) -> DispatchResult {
        // check if caller has CC_PROJECT_OWNER role
        ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);

        let new_id = LastID::get() + 1;
        let new_project = ProjectStruct::<<T as frame_system::Config>::AccountId>::new(caller, new_id, standard, filehash);
        <ProjectById<T>>::insert(new_id, new_project);
        LastID::mutate(|x| *x = x.checked_add(1).unwrap());
        Ok(())
    }

    pub fn sign_pdd(caller: T::AccountId, proj_id: u32) -> DispatchResult {
        ProjectById::<T>::try_mutate(
            proj_id, |project_to_mutate| -> DispatchResult {
                ensure!(project_to_mutate.is_some(), Error::<T>::AccountNotOwner);
                Self::change_project_state(&mut project_to_mutate.as_mut().unwrap(), caller)?;
                Ok(())
         })?;
        Ok(())
    }

    pub fn impl_create_annual_report(caller: T::AccountId, proj_id: u32, filehash: H256){

    }

    pub fn change_project_state(project: &mut ProjectStruct<T::AccountId>, caller: T::AccountId) -> DispatchResult {
        match &mut project.get_standard() {
            // Project Owner submits PDD (changing status to Registration) => 
            // => Auditor Approves PDD => Standard Certifies PDD => Registry Registers PDD (changing status to Issuance)
            Standard::GoldStandard  => {
                match project.state {
                    project::PROJECT_OWNER_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                        project.state = project::AUDITOR_SIGN_PENDING;
                        project.status = project::ProjectStatus::Registration;
                        project.signatures.push(caller);
                    },
                    project::AUDITOR_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                        project.state = project::STANDARD_SIGN_PENDING;
                        project.signatures.push(caller);
                    },
                    project::STANDARD_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                        project.state = project::REGISTRY_SIGN_PENDING;
                        project.signatures.push(caller);
                    },
                    project::REGISTRY_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                        project.state = project::REGISTERED;
                        project.status = project::ProjectStatus::Issuance;
                        project.signatures.push(caller);
                    },
                    _ => ensure!(false, Error::<T>::InvalidState)
                }
                Ok(())
            }
        }
    }

    #[cfg(test)]
    pub fn get_proj_by_id(id: u32) -> Option<ProjectStruct<T::AccountId>> {
        ProjectById::<T>::get(id)
    }
}