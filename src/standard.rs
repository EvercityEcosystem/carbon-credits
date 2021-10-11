use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum Standard {
    GoldStandard,
}

impl Default for Standard {
    fn default() -> Standard {
        Standard::GoldStandard
    }
}