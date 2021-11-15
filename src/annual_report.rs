use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use crate::required_signers::RequiredSigner;
use pallet_evercity_filesign::FileId;

pub type AnnualReportStateMask = u16;
pub const REPORT_PROJECT_OWNER_SIGN_PENDING: AnnualReportStateMask = 1;
pub const REPORT_AUDITOR_SIGN_PENDING: AnnualReportStateMask = 2;
pub const REPORT_STANDARD_SIGN_PENDING: AnnualReportStateMask = 4;
pub const REPORT_INVESTOR_SIGN_PENDING: AnnualReportStateMask = 8;
pub const REPORT_REGISTRY_SIGN_PENDING: AnnualReportStateMask = 16;
pub const REPORT_ISSUED: AnnualReportStateMask = 32;

/// Generic annual report implementation
pub type AnnualReportStruct<AccountId, T, Balance> = AnnualReportStructT<AccountId, <T as pallet_timestamp::Config>::Moment, Balance>;

/// Main annual report implementation
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct AnnualReportStructT<AccountId, Moment, Balance> where Balance: Clone, AccountId: PartialEq {
    pub file_id: FileId,
    pub state: AnnualReportStateMask,
    #[codec(compact)]
    create_time: Moment,
    carbon_credits_count: Balance,
    carbon_credits_released: bool,
    carbon_credits_meta: CarbonCreditsMeta,
    required_signers: Vec<RequiredSigner<AccountId>>,
}

impl<AccountId, Moment, Balance> AnnualReportStructT<AccountId, Moment, Balance> where Balance: Clone, AccountId: PartialEq {
    pub fn new(file_id: FileId, carbon_credits_count: Balance, create_time: Moment) -> Self {
        AnnualReportStructT{
            file_id,
            state: REPORT_PROJECT_OWNER_SIGN_PENDING,
            required_signers: Vec::new(),
            create_time,
            carbon_credits_count,
            carbon_credits_released: false,
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

    pub fn is_full_signed(&self) -> bool {
        self.state == REPORT_ISSUED
    }

    pub fn assign_required_signer(&mut self, signer: RequiredSigner<AccountId>) {
        self.required_signers.push(signer);
    }

    pub fn is_required_signer(&self, signer: RequiredSigner<AccountId>) -> bool {
        self.required_signers.iter().any(|(acc, role)| *acc == signer.0 && *role == signer.1)
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