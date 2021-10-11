#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
    dispatch::{
        DispatchResult, 
        DispatchError, 
        Vec,
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
pub mod state;
#[cfg(test)]
pub mod mock;
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
        pub fn sign_project(origin, proj_id: u32) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::sign_pdd(caller, proj_id)?;
            Ok(())
        }
    }
}

// Atomic operations here
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
                // let result = project_to_mutate.as_mut().unwrap().change_project_state(caller);

                Self::change_project_state(&mut project_to_mutate.as_mut().unwrap(), caller)?;
                // if let Err(err) = result {
                //     ensure!(false, Self::convert_project_err_to_module_err(&err));
                // }
                Ok(())
         })?;
        Ok(())
    }

    pub fn change_project_state(project: &mut ProjectStruct<T::AccountId>, caller: T::AccountId) -> DispatchResult {
        match &mut project.standard {
            // Project Owner submits PDD (changing status to Registration) => 
            // => Auditor Approves PDD => Standard Certifies PDD => Registry Registers PDD (changing status to Issuance)
            Standard::GoldStandard  => {
                match project.state {
                    state::PROJECT_OWNER_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                        project.state = state::AUDITOR_SIGN_PENDING;
                        project.status = project::ProjectStatus::Registration;
                        project.signatures.push(caller);
                    },
                    state::AUDITOR_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                        project.state = state::STANDARD_SIGN_PENDING;
                        project.signatures.push(caller);
                    },
                    state::STANDARD_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotAuditor);
                        project.state = state::REGISTRY_SIGN_PENDING;
                        project.signatures.push(caller);
                    },
                    state::REGISTRY_SIGN_PENDING => {
                        ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotAuditor);
                        project.state = state::REGISTERED;
                        project.status = project::ProjectStatus::Issuance;
                        project.signatures.push(caller);
                    },
                    _ => ensure!(false, Error::<T>::InvalidState)
                }
                Ok(())
            },
            _ => {
                ensure!(false, Error::<T>::InvalidStandard); 
                Ok(())
            },
        }
    }

    // pub fn submit_pdd_for_review(caller: T::AccountId, proj_id: u32) {
    // }

    // pub fn approve_pdd(caller: T::AccountId, proj_id: u32) {
    // }

    // pub fn certify_pdd(caller: T::AccountId, proj_id: u32) {
    // }

    // pub fn register_pdd(caller: T::AccountId, proj_id: u32) {
    // }

    // pub fn request_pdd_for_verification(caller: T::AccountId, proj_id: u32) {
    // }

    // pub fn submit_pdd_verification(caller: T::AccountId, proj_id: u32) {
    // }

    // pub fn approve_carbon_credit_issuance(caller: T::AccountId, proj_id: u32) {
    // }

    // pub fn issue_carbon_credit(caller: T::AccountId, proj_id: u32) {
    // }

    // fn convert_project_err_to_module_err(err: &ProjectError) -> Error<T> {
    //     match err {
    //         ProjectError::InvalidStandard => Error::<T>::InvalidAction,
    //         ProjectError::NotAnOwner => Error::<T>::AccountNotOwner,
    //         _ => Error::<T>::InvalidAction
    //     }
    // }


    #[cfg(test)]
    pub fn get_proj_by_id(id: u32) -> Option<ProjectStruct<T::AccountId>> {
        ProjectById::<T>::get(id)
    }
}


// fn process_request<T, K>(func: impl FnOnce(K) -> DispatchResult, arg: K) -> DispatchResult where T: Config {
//     func(arg)
// }