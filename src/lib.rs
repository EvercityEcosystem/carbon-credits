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


pub mod account;
pub mod standard;
#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;


pub trait Config: frame_system::Config {}

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        //LastID: u32;
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
    }
}

// Atomic operations here
impl<T: Config> Module<T> {

}