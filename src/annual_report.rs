use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use crate::file_hash::*;

pub type AnnualReportStateMask = u16;
pub const REPORT_PROJECT_OWNER_SIGN_PENDING: AnnualReportStateMask = 1;
pub const REPORT_AUDITOR_SIGN_PENDING: AnnualReportStateMask = 2;
pub const REPORT_STANDARD_SIGN_PENDING: AnnualReportStateMask = 4;
pub const REPORT_INVESTOR_SIGN_PENDING: AnnualReportStateMask = 8;
pub const REPORT_REGISTRY_SIGN_PENDING: AnnualReportStateMask = 16;
pub const REPORT_ISSUED: AnnualReportStateMask = 32;

pub type AnnualReportStruct<AccountId, T, Balance> = AnnualReportStructT<AccountId, <T as pallet_timestamp::Config>::Moment, Balance>;

pub type CarbonCreditsBalance = u32;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct AnnualReportStructT<AccountId, Moment, Balance> where Balance: Clone {
    pub filehash: H256,
    pub state: AnnualReportStateMask,
    pub signatures: Vec<AccountId>,
    #[codec(compact)]
    create_time: Moment,
    carbon_credits_count: Balance,
    carbon_credits_released: bool,
    carbon_credits_meta: CarbonCreditsMeta,
}

impl<AccountId, Moment, Balance> AnnualReportStructT<AccountId, Moment, Balance> where Balance: Clone {
    pub fn new(filehash: H256, carbon_credits_count: Balance, create_time: Moment) -> Self {
        AnnualReportStructT{
            filehash,
            state: REPORT_PROJECT_OWNER_SIGN_PENDING,
            signatures: Vec::new(),
            carbon_credits_count,
            carbon_credits_released: false,
            create_time,
            carbon_credits_meta: CarbonCreditsMeta::new(),
        }
    }

    pub fn is_carbon_credits_released(&self) -> bool {
        self.carbon_credits_released
    }

    pub fn set_carbon_credits_released(&mut self) {
        self.carbon_credits_released = true;
    }

    pub fn carbon_credits_count(&self) -> Balance {
        self.carbon_credits_count.clone()
    }

    pub fn set_metadata(&mut self, name: Vec<u8>, symbol: Vec<u8>, decimals: u8){
        self.carbon_credits_meta.set_metadata(name, symbol, decimals);
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsMeta {
    name: Vec<u8>,
    symbol: Vec<u8>,
    decimals: u8,
}

impl CarbonCreditsMeta {
    pub fn new() -> Self{
        CarbonCreditsMeta {
            name: Vec::new(), 
            symbol: Vec::new(),
            decimals: 0
        }
    }

    pub fn is_metadata_valid(&self) -> bool {
        self.name.len() != 0 && self.symbol.len() != 0 
    }

    pub fn set_metadata(&mut self, name: Vec<u8>, symbol: Vec<u8>, decimals: u8){
        self.name = name;
        self.symbol = symbol;
        self.decimals = decimals;
    }
}