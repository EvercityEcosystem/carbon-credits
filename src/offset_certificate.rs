use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsOffsetCertificate<AssetId, Balance> {
    pub asset_id: AssetId,
    pub offset_amount: Balance,
}

impl<AssetId, Balance> CarbonCreditsOffsetCertificate<AssetId, Balance>{
    pub fn new(asset_id: AssetId, offset_amount: Balance) -> Self {
        Self { 
            asset_id, 
            offset_amount
        }
    }
}