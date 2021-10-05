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

// use frame_support::traits::IntegrityTest;

use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
};

use project::ProjectStruct;
use standard::Standard;
use account::*;

pub mod account;
pub mod standard;
pub mod project;
#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;

pub trait Config: frame_system::Config {}

// pub trait Config: frame_system::Config + pallet_timestamp::Config {
//     // type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
//     // type BurnRequestTtl: Get<u32>;
//     // type MintRequestTtl: Get<u32>;
//     // type MaxMintAmount: Get<EverUSDBalance>;
//     // type TimeStep: Get<BondPeriod>;
//     // type WeightInfo: WeightInfo;
//     // type OnAddAccount: OnAddAccount<Self::AccountId, Self::Moment>;
//     // type OnAddBond: OnAddBond<Self::AccountId, Self::Moment, Self::Hash>;
// }

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        Fuse get(fn fuse)
            build(|config| !config.genesis_account_registry.is_empty()):
            bool;

        /// Storage map for accounts, their roles and corresponding info
        AccountRegistry
            get(fn account_registry)
            config(genesis_account_registry):
            map hasher(blake2_128_concat) T::AccountId => CarbonCreditAccountStruct;

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
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        pub fn create_project(origin, standard: Standard) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            let new_id = LastID::get() + 1;
            let new_project = ProjectStruct::<<T as frame_system::Config>::AccountId>::new(caller, new_id, standard);
            <ProjectById<T>>::insert(new_id, new_project);
            LastID::mutate(|x| *x = x.checked_add(1).unwrap());
            Ok(())
        }

        // #[weight = 10_000]
        // pub fn submit_project_for_rewiev(origin, proj_id: u32) -> DispatchResult {
        //     let caller = ensure_signed(origin)?;

            
        //     Ok(())
        // }

        #[weight = 10_000]
        fn account_add_with_role_and_data(origin, who: T::AccountId, role: u8, #[compact] identity: u64) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotAuthorized);
            // ensure!(!AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountToAddAlreadyExists);
            // ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);

            // Self::account_add( &who, EvercityAccountStructT { roles: role, identity, create_time: Timestamp::<T>::get() } );

            // Self::deposit_event(RawEvent::AccountAdd(caller, who, role, identity));
            Ok(())
        }
    }
}

// Atomic operations here
impl<T: Config> Module<T> {
    fn account_add(account: &T::AccountId, mut data: CarbonCreditAccountStruct) {
        // data.create_time = Timestamp::<T>::get();
        AccountRegistry::<T>::insert(account, &data);
        // T::OnAddAccount::on_add_account(account, &data);
    }

    /// <pre>
    /// Method: account_is_master(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Master role
    /// </pre>
    pub fn account_is_master(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).role == MASTER
    }

    pub fn submit_pdd_for_rewiev(caller: T::AccountId, proj_id: u32) {

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

    pub fn issue_carbon_Credit(caller: T::AccountId, proj_id: u32) {

    }

    #[cfg(test)]
    pub fn get_proj_by_id(id: u32) -> Option<ProjectStruct<T::AccountId>> {
        ProjectById::<T>::get(id)
    }
}

fn process_request<T, K>(func: impl FnOnce(K) -> DispatchResult, arg: K) -> DispatchResult where T: Config {
    func(arg)
}