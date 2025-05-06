use alloy_primitives::{keccak256, Address, Bytes, B256};
use serde::{Deserialize, Serialize};

use crate::{Error, Origin, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ResponseTemplate {
    Offset { begin: u64, length: u64 },
    Prefix { prefix: Bytes, length: u64 },
}

impl ResponseTemplate {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            ResponseTemplate::Offset { begin, length } => {
                let mut res = Vec::new();
                res.push(0u8);
                res.push(1u8);
                res.extend_from_slice(&begin.to_be_bytes());
                res.extend_from_slice(&length.to_be_bytes());
                res
            }
            ResponseTemplate::Prefix { prefix, length } => {
                let mut res = Vec::new();
                res.push(0u8);
                res.push(2u8);
                res.push(prefix.len() as u8);
                res.extend_from_slice(prefix.as_ref());
                res.extend_from_slice(&length.to_be_bytes());
                res
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInfo {
    pub request: Bytes,
    pub remote_addr: String,
    pub server_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestTarget {
    pub client: Address,
    pub prover_id: B256,
    pub submit_network_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub version: u8,
    pub request_info: RequestInfo,
    pub response_template: Vec<ResponseTemplate>,
    pub target: RequestTarget,
    pub origin: Origin,
}

impl Request {
    pub const VERSION: u8 = 1;

    pub fn request_hash(&self) -> B256 {
        let mut hasher = alloy_primitives::Keccak256::new();

        hasher.update(&self.version.to_le_bytes());
        hasher.update(&self.request_info.request);
        hasher.update(self.request_info.remote_addr.as_bytes());
        hasher.update(self.request_info.server_name.as_bytes());

        for template in &self.response_template {
            hasher.update(template.as_bytes());
        }

        hasher.update(self.target.client);
        hasher.update(self.target.prover_id);
        hasher.update(&self.target.submit_network_id.to_be_bytes());

        hasher.update(self.origin.nonce().to_be_bytes());

        hasher.finalize()
    }

    // fn apikey_request_id(&self) -> Result<B256> {
    //     let request_hash = self.request_hash();

    //     let mut res = Vec::with_capacity(20 + 32 + 8);

    //     res.extend_from_slice(self.dapp()?.as_slice());
    //     res.extend_from_slice(request_hash.as_slice());
    //     res.extend_from_slice(&self.origin.nonce().to_be_bytes());

    //     Ok(keccak256(&res))
    // }

    fn secp256k1_request_id(&self) -> Result<B256> {
        let mut res = Vec::with_capacity(20 + 8);

        res.extend_from_slice(self.dapp()?.as_slice());
        res.extend_from_slice(&self.origin.nonce().to_be_bytes());

        Ok(keccak256(&res))
    }

    pub fn request_id(&self) -> Result<B256> {
        match &self.origin {
            Origin::None { nonce: _ } => Err(Error::MustSetOrigin),
            // Origin::ApiKey(_) => self.apikey_request_id(),
            Origin::Secp256k1(_) => self.secp256k1_request_id(),
        }
    }

    pub fn dapp(&self) -> Result<B256> {
        match &self.origin {
            Origin::None { nonce: _ } => Err(Error::MustSetOrigin),
            // Origin::ApiKey(f) => Ok(f.dapp()),
            Origin::Secp256k1(f) => Ok(f.dapp(self.request_hash())?),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub response: Vec<Bytes>,
    pub request_id: B256,
    pub target: RequestTarget,
    pub dapp: B256,
    #[serde(default)]
    pub proof: Bytes,
}

impl Response {
    pub fn from_request(request: &Request, response: Vec<Bytes>) -> Result<Self> {
        Ok(Self {
            response,
            request_id: request.request_id()?,
            target: request.target.clone(),
            dapp: request.dapp()?,
            proof: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id() {
        extern crate std;

        let request_s = include_str!("../testdata/request.json");
        let request: Request = serde_json::from_str(request_s).unwrap();
        std::println!("{:?}", request.dapp());

        let request_s1 = include_str!("../testdata/request1.json");
        let request1: Request = serde_json::from_str(request_s1).unwrap();
        std::println!("{:?}", request1.dapp());

        let request_s2 = include_str!("../testdata/request2.json");
        let request2: Request = serde_json::from_str(request_s2).unwrap();
        std::println!("{:?}", request2.dapp());
    }
}
