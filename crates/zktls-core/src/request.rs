use alloc::{string::String, vec::Vec};
use alloy::{
    primitives::{Bytes, B256, U256},
    sol_types::SolValue,
};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestDataFull {
    pub encrypted_offset: U256,
    pub fields: Vec<u64>,
    pub values: Vec<Bytes>,
    pub remote: String,
    pub server_name: String,
    pub request_template_hash: B256,
}

impl RequestDataFull {
    pub fn encode(self) -> Vec<u8> {
        (
            self.encrypted_offset,
            self.fields,
            self.values,
            self.remote,
            self.server_name,
            self.request_template_hash,
        )
            .abi_encode_sequence()
    }

    pub fn decode(bytes: Vec<u8>) -> Result<Self> {
        type StructType = (U256, Vec<u64>, Vec<Bytes>, String, String, B256);

        let (encrypted_offset, fields, values, remote, server_name, request_template_hash) =
            StructType::abi_decode_sequence(&bytes, true).map_err(Error::AlloySolTypesError)?;

        Ok(Self {
            encrypted_offset,
            fields,
            values,
            remote,
            server_name,
            request_template_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use alloy::hex::FromHex;

    use super::*;

    #[test]
    fn test_encode_decode() {
        let request_data = RequestDataFull {
            encrypted_offset: U256::ZERO,
            fields: Vec::from([0, 1, 2]),
            values: Vec::from([Bytes::from("hello"), Bytes::from("world")]),
            remote: "hello".into(),
            server_name: "world".into(),
            request_template_hash: B256::ZERO,
        };

        let encoded = request_data.clone().encode();
        let decoded = RequestDataFull::decode(encoded).unwrap();

        assert_eq!(request_data, decoded);
    }

    #[test]
    fn test_hash() {
        let request_data = RequestDataFull::decode(
            Bytes::from_hex(include_str!("../testdata/request_data.txt"))
                .unwrap()
                .to_vec(),
        )
        .unwrap();

        assert_eq!(
            request_data,
            RequestDataFull {
                encrypted_offset: U256::from(1),
                fields: Vec::from([1]),
                values: Vec::from([Bytes::from(b"123")]),
                remote: "hello".into(),
                server_name: "world".into(),
                request_template_hash: B256::ZERO,
            }
        );
    }
}
