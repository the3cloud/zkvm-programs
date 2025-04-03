mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Request {
    request: zktls_program_core::Request,
}

#[wasm_bindgen]
impl Request {
    pub fn from_value(value: JsValue) -> Result<Self, JsValue> {
        let request = serde_wasm_bindgen::from_value(value)?;
        Ok(Self { request })
    }

    pub fn to_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.request).map_err(|e| JsValue::from(e.to_string()))
    }

    pub fn request_hash(&self) -> Result<Box<[u8]>, JsValue> {
        let hash = self.request.request_hash();
        Ok(hash.to_vec().into_boxed_slice())
    }

    pub fn request_id(&self) -> Result<Box<[u8]>, JsValue> {
        let id = self
            .request
            .request_id()
            .map_err(|e| JsValue::from(e.to_string()))?;
        Ok(id.to_vec().into_boxed_slice())
    }

    pub fn dapp(&self) -> Result<Box<[u8]>, JsValue> {
        let dapp = self
            .request
            .dapp()
            .map_err(|e| JsValue::from(e.to_string()))?;
        Ok(dapp.to_vec().into_boxed_slice())
    }
}
