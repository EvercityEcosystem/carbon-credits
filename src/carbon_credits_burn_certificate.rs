use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsBurnCertificate<AssetId, Balance> {
    pub asset_id: AssetId,
    pub burned_amount: Balance,
}

impl<AssetId, Balance> CarbonCreditsBurnCertificate<AssetId, Balance>{
    pub fn new(asset_id: AssetId, burned_amount: Balance) -> Self {
        Self { 
            asset_id, 
            burned_amount
        }
    }
}