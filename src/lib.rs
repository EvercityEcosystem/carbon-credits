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
use project::ProjectStruct;
use standard::Standard;

// pub mod account;
pub mod standard;
pub mod project;
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
        InvalidAction,
        AccountNotExist,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        pub fn create_project(origin, standard: Standard) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::create_pdd(caller, standard)?;
            Ok(())
        }
    }
}

// Atomic operations here
impl<T: Config> Module<T> {
    pub fn create_pdd(caller: T::AccountId, standard: Standard) -> DispatchResult {
        ensure!(accounts::Module::<T>::account_is_project_owner(&caller), Error::<T>::AccountNotOwner);
        let new_id = LastID::get() + 1;
        let new_project = ProjectStruct::<<T as frame_system::Config>::AccountId>::new(caller, new_id, standard);
        <ProjectById<T>>::insert(new_id, new_project);
        LastID::mutate(|x| *x = x.checked_add(1).unwrap());
        Ok(())
    }

    pub fn submit_pdd_for_review(caller: T::AccountId, proj_id: u32) {
    }

    pub fn approve_pdd(caller: T::AccountId, proj_id: u32) {
    }

    pub fn certify_pdd(caller: T::AccountId, proj_id: u32) {
    }

    pub fn register_pdd(caller: T::AccountId, proj_id: u32) {
    }

    pub fn request_pdd_for_verification(caller: T::AccountId, proj_id: u32) {
    }

    pub fn submit_pdd_verification(caller: T::AccountId, proj_id: u32) {
    }

    pub fn approve_carbon_credit_issuance(caller: T::AccountId, proj_id: u32) {
    }

    pub fn issue_carbon_credit(caller: T::AccountId, proj_id: u32) {
    }

    #[cfg(test)]
    pub fn get_proj_by_id(id: u32) -> Option<ProjectStruct<T::AccountId>> {
        ProjectById::<T>::get(id)
    }
}

fn process_request<T, K>(func: impl FnOnce(K) -> DispatchResult, arg: K) -> DispatchResult where T: Config {
    func(arg)
}