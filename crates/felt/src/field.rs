use crate::Felt;
use starknet_types_core::felt::Felt as SnFelt;

impl From<SnFelt> for Felt {
    fn from(value: SnFelt) -> Self {
        Self(value.to_bytes_be())
    }
}

impl From<&SnFelt> for Felt {
    fn from(value: &SnFelt) -> Self {
        Self(value.to_bytes_be())
    }
}

impl From<Felt> for SnFelt {
    fn from(value: Felt) -> Self {
        Self::from_bytes_be(&value.0)
    }
}
impl From<&Felt> for SnFelt {
    fn from(value: &Felt) -> Self {
        Self::from_bytes_be(&value.0)
    }
}
