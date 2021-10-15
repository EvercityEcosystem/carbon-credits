use fixed_hash::construct_fixed_hash;
use frame_support::{
    codec::{Decode, Encode},
};

construct_fixed_hash! {
    /// 256 bit hash type for signing files
    #[derive(Encode, Decode)]
    pub struct H256(32);
}