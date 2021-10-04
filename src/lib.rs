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

use frame_support::traits::IntegrityTest;

use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
};

use project::ProjectStruct;

pub mod account;
pub mod standard;
pub mod project;
#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;


pub trait Config: frame_system::Config {}

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        ProjectById
            get(fn project_by_id):
            map hasher(blake2_128_concat) u32 => ProjectStruct<T::AccountId>;
            
        LastID: u32;
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        AddressNotAuditor,
        AddressNotOwner
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

        #[weight = 10_000]
        pub fn test1(origin) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Self::test()
        }
    }
}

// Atomic operations here
impl<T: Config> Module<T> {
    pub fn test() -> DispatchResult {
        LastID::try_mutate(|x| -> DispatchResult {
            *x = 1;
            Ok(())
        })
    }
}

fn process_request<T, K>(func: impl FnOnce(K) -> DispatchResult, arg: K) -> DispatchResult where T: Config {
    func(arg)
}