use std::{
    io::{Read, Write},
    sync::Arc,
};

use alloy::sol_types::SolValue;
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use t3zktls_program_core::{GuestInputResponse, RequestFull};
use t3zktls_replayable_tls::{crypto_provider, set_random, ReplayStream, ReplayTimeProvider};

pub fn execute(request: Vec<u8>, response: GuestInputResponse) -> Vec<u8> {
    let mut stream = ReplayStream::new(response.stream.clone());
    let time_provider = ReplayTimeProvider::new(&response.time);
    set_random(response.random);

    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    let crypto_provider = crypto_provider();

    let config =
        ClientConfig::builder_with_details(Arc::new(crypto_provider), Arc::new(time_provider))
            .with_safe_default_protocol_versions()
            .expect("Failed to set protocol versions")
            .with_root_certificates(root_store)
            .with_no_client_auth();

    let request_data = RequestFull::decode(&request).expect("Failed to decode request");

    let server_name = String::from(&request_data.server_name)
        .try_into()
        .expect("Failed to convert server name");

    let mut tls_stream =
        ClientConnection::new(Arc::new(config), server_name).expect("Failed to create TLS stream");

    let mut tls = rustls::Stream::new(&mut tls_stream, &mut stream);

    let request_data = request_data.request;

    tls.write_all(&request_data).expect("Failed to write data");

    let mut buf = Vec::new();
    tls.read_to_end(&mut buf).expect("Failed to read data");

    let mut serialized_request = Vec::new();
    ciborium::into_writer(&request, &mut serialized_request).expect("Failed to serialize request");

    // TODO: Match response in buf;
    let response_data = response.filtered_responses.abi_encode();

    response_data
}
