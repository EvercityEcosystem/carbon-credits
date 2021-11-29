use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsBurnCertificate<AssetId, Balance> {
    pub asset_id: AssetId,
    pub burn_amount: Balance,
}

impl<AssetId, Balance> CarbonCreditsBurnCertificate<AssetId, Balance>{
    pub fn new(asset_id: AssetId, burn_amount: Balance) -> Self {
        Self { 
            asset_id, 
            burn_amount
        }
    }
}