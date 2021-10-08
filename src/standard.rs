// pub type Standard = u8;

// pub const GOLD_STANDARD: Standard = 1u8;
// pub const OTHER_STANDARD: Standard = 2u8;

// pub struct StandardValidator;
use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum Standard {
    GoldStandard
}

impl Default for Standard {
    fn default() -> Standard {
        Standard::GoldStandard
    }
}