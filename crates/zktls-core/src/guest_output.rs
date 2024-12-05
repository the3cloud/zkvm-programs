use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuestOutput {
    pub response_data: Vec<u8>,
    pub request_hash: [u8; 32],
}

impl GuestOutput {
    pub fn decode(bytes: Vec<u8>) -> Result<Self> {
        let mut bytes = bytes;

        let bytes_len = bytes.len();

        if bytes_len < 32 {
            return Err(Error::InvalidBytesLength);
        }

        let request_hash = bytes[bytes_len - 32..]
            .try_into()
            .map_err(|_| Error::TryFromSliceError)?;
        bytes.truncate(bytes_len - 32);

        Ok(Self {
            request_hash,
            response_data: bytes,
        })
    }

    pub fn encode(self) -> Vec<u8> {
        let mut bytes = self.response_data;

        bytes.extend_from_slice(&self.request_hash);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guest_output() {
        let guest_output = GuestOutput {
            request_hash: [10; 32],
            response_data: Vec::from([1, 2, 3]),
        };

        let bytes = guest_output.clone().encode();
        let guest_output_2 = GuestOutput::decode(bytes).unwrap();

        assert_eq!(guest_output, guest_output_2);
    }
}
