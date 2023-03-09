use codec::{Decode, Encode};
use frame_support::weights::Weight;

pub type VmId = u8;

#[derive(Default, PartialEq, Eq, Clone, Encode, Decode, scale_info::TypeInfo)]
pub struct XvmContext {
    /// Identifier (should be unique for each VM in tuple).
    pub id: VmId,
    /// Max allowed weight for the call
    pub max_weight: Weight,
    /// Encoded VM execution environment.
    pub env: Option<Vec<u8>>,
}

fn main() {
    let context = XvmContext {
        id: 31,
        max_weight: Weight::from_parts(4006871040u64, 131072u64),
        env: None,
    };
    let encoded = context.encode();
    let hexx = hex::encode(&encoded);
    println!("{:X?}", hexx);
}