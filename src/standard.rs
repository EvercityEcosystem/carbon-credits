use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Standard {
    GOLD_STANDARD,
}

impl Default for Standard {
    fn default() -> Standard {
        Standard::GOLD_STANDARD
    }
}