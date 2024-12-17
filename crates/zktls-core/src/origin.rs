use alloy::primitives::{keccak256, PrimitiveSignature, B256, B512};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyOrigin {
    pub key: B256,
    pub nonce: u64,
    #[serde(skip)]
    pub salt: B256,
}

impl ApiKeyOrigin {
    pub fn dapp(&self) -> B256 {
        let mut res = Vec::with_capacity(64);

        res.extend_from_slice(self.key.as_slice());
        res.extend_from_slice(self.salt.as_slice());

        let hash = keccak256(&res);

        let mut address = [0u8; 32];
        address.copy_from_slice(&hash[0..32]);

        address.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secp256k1Origin {
    pub signature: B512,
    pub v: bool,
    pub nonce: u64,
}

impl Secp256k1Origin {
    pub fn dapp(&self, hash: B256) -> Result<B256> {
        let signature =
            PrimitiveSignature::from_bytes_and_parity(self.signature.as_slice(), self.v);

        let address = signature
            .recover_from_prehash(&hash)
            .map_err(Error::SignatureError)?;

        let bytes = address.to_sec1_bytes();

        let bb = keccak256(bytes);

        Ok(B256::from(bb))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Origin {
    ApiKey(ApiKeyOrigin),
    Secp256k1(Secp256k1Origin),
}

impl Origin {
    pub fn nonce(&self) -> u64 {
        match self {
            Origin::ApiKey(ApiKeyOrigin { nonce, .. }) => *nonce,
            Origin::Secp256k1(Secp256k1Origin { nonce, .. }) => *nonce,
        }
    }

    pub fn dapp(&self, hash: B256) -> Result<B256> {
        match self {
            Origin::ApiKey(e) => Ok(e.dapp()),
            Origin::Secp256k1(e) => e.dapp(hash),
        }
    }
}
