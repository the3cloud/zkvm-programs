use alloy::{
    primitives::{keccak256, Address, Bytes, B256},
    sol_types::SolValue,
};
use serde::{Deserialize, Serialize};

use crate::{Error, Origin, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ResponseTemplate {
    Offset { begin: u64, length: u64 },
    Regex { pattern: String },
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
            ResponseTemplate::Regex { pattern } => {
                let mut res = Vec::new();
                res.push(0u8);
                res.push(2u8);
                res.push(pattern.len() as u8);
                res.extend_from_slice(pattern.as_bytes());
                res
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestClient {
    pub client: Address,
    pub max_gas_price: u64,
    pub max_gas_limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub request: Bytes,
    pub remote_addr: String,
    pub server_name: String,
    pub response_template: Vec<ResponseTemplate>,
    pub origin: Origin,
    pub client: RequestClient,
}

impl Request {
    /// this function must be called used internal appid
    pub fn set_appid(&mut self, appid: &[u8]) {
        if let Origin::ApiKey(f) = &mut self.origin {
            let salt = keccak256(appid);
            f.salt = salt;
        }
    }

    pub fn request_hash(&self) -> B256 {
        let mut hasher = alloy::primitives::Keccak256::new();

        hasher.update(&self.request);
        hasher.update(self.remote_addr.as_bytes());
        hasher.update(self.server_name.as_bytes());

        for template in &self.response_template {
            hasher.update(template.as_bytes());
        }

        hasher.update(self.client.client);
        hasher.update(self.client.max_gas_price.to_be_bytes());
        hasher.update(self.client.max_gas_limit.to_be_bytes());

        hasher.finalize()
    }

    fn apikey_request_id(&self) -> Result<B256> {
        let request_hash = self.request_hash();

        let mut res = Vec::with_capacity(20 + 32 + 8);

        res.extend_from_slice(self.dapp()?.as_slice());
        res.extend_from_slice(request_hash.as_slice());
        res.extend_from_slice(&self.origin.nonce()?.to_be_bytes());

        Ok(keccak256(&res))
    }

    fn secp256k1_request_id(&self) -> Result<B256> {
        let mut res = Vec::with_capacity(20 + 8);

        res.extend_from_slice(self.dapp()?.as_slice());
        res.extend_from_slice(&self.origin.nonce()?.to_be_bytes());

        Ok(keccak256(&res))
    }

    pub fn request_id(&self) -> Result<B256> {
        match &self.origin {
            Origin::None => Err(Error::MustSetOrigin),
            Origin::ApiKey(_) => self.apikey_request_id(),
            Origin::Secp256k1(_) => self.secp256k1_request_id(),
        }
    }

    pub fn dapp(&self) -> Result<B256> {
        match &self.origin {
            Origin::None => Err(Error::MustSetOrigin),
            Origin::ApiKey(f) => Ok(f.dapp()),
            Origin::Secp256k1(f) => Ok(f.dapp(self.request_hash())?),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    #[serde(with = "serde_bytes")]
    pub response: Vec<u8>,
    pub request_id: B256,
    pub client: Address,
    pub dapp: B256,
    pub max_gas_price: u64,
    pub max_gas_limit: u64,
    #[serde(with = "serde_bytes")]
    pub proof: Vec<u8>,
    #[serde(default)]
    pub prover_id: B256,
}

impl Response {
    pub fn from_request(request: &Request, response: Vec<u8>) -> Result<Self> {
        Ok(Self {
            response,
            request_id: request.request_id()?,
            client: request.client.client,
            dapp: request.dapp()?,
            max_gas_price: request.client.max_gas_price,
            max_gas_limit: request.client.max_gas_limit,
            proof: Default::default(),
            prover_id: Default::default(),
        })
    }

    pub fn abi_encode(self) -> Vec<u8> {
        (
            self.request_id,
            self.client,
            self.dapp,
            self.max_gas_price,
            self.max_gas_limit,
            self.response,
        )
            .abi_encode_sequence()
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

        std::println!("{:?}", request);
    }
}
