use alloc::{string::String, vec::Vec};
use alloy::{
    primitives::{Bytes, B256, U256},
    sol_types::SolValue,
};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestFull {
    pub encrypted_offset: U256,
    pub fields: Vec<u64>,
    pub values: Vec<Bytes>,
    pub remote: String,
    pub server_name: String,
    pub request_template_hash: B256,
    pub request_template: Vec<u8>,
}

impl RequestFull {
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

    pub fn decode(bytes: &[u8], request_template: Vec<u8>) -> Result<Self> {
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
            request_template,
        })
    }

    pub fn data(&self) -> Result<Vec<u8>> {
        let total_length = self.values.iter().map(|v| v.len()).sum::<usize>();

        let mut data = Vec::with_capacity(total_length);

        let mut offset = 0usize;

        for (idx, value) in self.fields.iter().zip(self.values.iter()) {
            let end = offset + *idx as usize;

            data.extend_from_slice(&self.request_template[offset..end]);
            data.extend_from_slice(&value.as_ref());

            offset = end;
        }

        data.extend_from_slice(&self.request_template[offset..]);

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use alloy::hex::FromHex;

    use super::*;

    #[test]
    fn test_encode_decode() {
        let request_data = RequestFull {
            encrypted_offset: U256::ZERO,
            fields: Vec::from([0, 1, 2]),
            values: Vec::from([Bytes::from("hello"), Bytes::from("world")]),
            remote: "hello".into(),
            server_name: "world".into(),
            request_template_hash: B256::ZERO,
            request_template: Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        };

        let encoded = request_data.clone().encode();
        let decoded = RequestFull::decode(&encoded, request_data.request_template.clone()).unwrap();

        assert_eq!(request_data, decoded);
    }

    #[test]
    fn test_hash() {
        let request_data = RequestFull::decode(
            &Bytes::from_hex(include_str!("../testdata/request_data.txt")).unwrap(),
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        )
        .unwrap();

        assert_eq!(
            request_data,
            RequestFull {
                encrypted_offset: U256::from(1),
                fields: Vec::from([1]),
                values: Vec::from([Bytes::from(b"123")]),
                remote: "hello".into(),
                server_name: "world".into(),
                request_template_hash: B256::ZERO,
                request_template: Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            }
        );
    }

    #[test]
    fn test_data() {
        let request_data = RequestFull::decode(
            &Bytes::from_hex(include_str!("../testdata/request_data.txt")).unwrap(),
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        )
        .unwrap();

        assert_eq!(
            request_data.data().unwrap(),
            Vec::from([0, 49, 50, 51, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
    }
}
