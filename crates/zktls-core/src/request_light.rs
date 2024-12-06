use alloc::vec::Vec;
use alloy::{
    primitives::{keccak256, Bytes, B256, U256},
    sol_types::SolValue,
};

use crate::{Error, Result};

pub struct RequestLight {
    pub encrypted_offset: U256,
    pub fields: Vec<u64>,
    pub values: Vec<Bytes>,

    pub remaining_bytes: Vec<u8>,
}

impl RequestLight {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        type StructType = (U256, Vec<u64>, Vec<Bytes>);

        let (encrypted_offset, fields, values) = StructType::abi_decode_sequence(bytes, false)
            .map_err(|e| Error::AlloySolTypesError(e))?;

        let segment_length_bytes: [u8; 32] = bytes[96..128]
            .try_into()
            .map_err(|_| Error::TryFromSliceError)?;

        let segment_length = U256::from_be_bytes(segment_length_bytes)
            .try_into()
            .map_err(|_| Error::TryFromSliceError)?;

        let remaining_bytes = bytes[segment_length..].to_vec();

        Ok(Self {
            encrypted_offset,
            fields,
            values,
            remaining_bytes,
        })
    }

    pub fn hash(&self) -> Result<B256> {
        let encrypted_offset: usize = self
            .encrypted_offset
            .try_into()
            .map_err(|_| Error::TryFromSliceError)?;

        let mut fields = self.fields.clone();
        let mut values = self.values.clone();

        fields.truncate(encrypted_offset);
        values.truncate(encrypted_offset);

        let bytes =
            (self.encrypted_offset, fields, values, &self.remaining_bytes).abi_encode_sequence();

        Ok(keccak256(&bytes).into())
    }
}

#[cfg(test)]
mod tests {
    use alloy::hex::FromHex;

    use super::*;

    #[test]
    fn test_hash() {
        let _ = env_logger::builder().is_test(true).try_init();

        let bytes = Bytes::from_hex(include_str!("../testdata/request_data.txt")).unwrap();

        let request_light = RequestLight::decode(&bytes).unwrap();

        let hash = request_light.hash().unwrap();

        assert_eq!(
            hash,
            B256::from_hex("0x5621772e4a5d88d4b8144d4621ecaebc9b44e5f53c8ef1f95c15525b804dc19b")
                .unwrap()
        );
    }
}
