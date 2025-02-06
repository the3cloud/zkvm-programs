use alloy::primitives::{keccak256, normalize_v, Address, FixedBytes, PrimitiveSignature, B256};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyOrigin {
    pub key: B256,
    #[serde(skip)]
    pub salt: B256,
}

impl ApiKeyOrigin {
    pub fn dapp(&self) -> Address {
        let mut res = Vec::with_capacity(64);

        res.extend_from_slice(self.key.as_slice());
        res.extend_from_slice(self.salt.as_slice());

        let hash = keccak256(&res);

        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[0..20]);

        Address::from(address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secp256k1Origin {
    pub signature: FixedBytes<65>,
}

impl Secp256k1Origin {
    pub fn dapp(&self, hash: B256) -> Result<Address> {
        let v = self.signature[64];

        let v = normalize_v(v as u64).ok_or(Error::InvalidNormalizeV)?;

        let signature = PrimitiveSignature::from_bytes_and_parity(self.signature.as_slice(), v);

        let address = signature
            .recover_address_from_prehash(&hash)
            .map_err(Error::SignatureError)?;

        Ok(address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Origin {
    None,
    ApiKey(ApiKeyOrigin),
    Secp256k1(Secp256k1Origin),
}

impl Origin {
    pub fn dapp(&self, hash: B256) -> Result<Address> {
        match self {
            Origin::None => Err(Error::MustSetOrigin),
            Origin::ApiKey(e) => Ok(e.dapp()),
            Origin::Secp256k1(e) => e.dapp(hash),
        }
    }
}
