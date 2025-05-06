use alloy_primitives::{Address, B256};
use clap::Parser;
use k256::{ecdsa::SigningKey, SecretKey};
use zktls_program_core::{
    Origin, Request, RequestInfo, RequestTarget, ResponseTemplate, Secp256k1Origin,
};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

#[derive(Debug, Parser)]
enum SubCmd {
    Sign {
        #[clap(long = "version", default_value_t = Request::VERSION)]
        version: u8,

        #[clap(long = "request.body")]
        request_body: String,

        #[clap(long = "request.addr")]
        request_addr: String,

        #[clap(long = "request.server")]
        request_server: String,

        #[clap(long = "response.prefix")]
        response_prefix: Vec<String>,

        #[clap(long = "response.prefix.length")]
        response_prefix_length: Vec<u32>,

        #[clap(long = "target.client")]
        client: Address,

        #[clap(long = "target.prover-id")]
        prover_id: B256,

        #[clap(long = "target.submit-network-id")]
        submit_network_id: u64,

        #[clap(long = "secp256k1.key")]
        secp256k1_key: B256,
    },
}

fn main() {
    let args = Args::parse();

    match args.subcmd {
        SubCmd::Sign {
            version,
            request_body,
            request_addr,
            request_server,
            response_prefix,
            response_prefix_length,
            client,
            prover_id,
            submit_network_id,
            secp256k1_key,
        } => {
            let request_info = RequestInfo {
                request: request_body.into(),
                remote_addr: request_addr.into(),
                server_name: request_server.into(),
            };

            let mut response_template = Vec::new();
            for (prefix, length) in response_prefix
                .into_iter()
                .zip(response_prefix_length.iter())
            {
                response_template.push(ResponseTemplate::Prefix {
                    prefix: prefix.into(),
                    length: *length as u64,
                });
            }

            let mut request = Request {
                request_info,
                response_template,
                origin: Origin::None,
                target: RequestTarget {
                    client,
                    prover_id,
                    submit_network_id,
                },
                version,
            };

            let request_hash = request.request_hash();

            // Convert the secp256k1_key to k256 SecretKey
            let secret_key = SecretKey::from_bytes(&secp256k1_key.0.into()).unwrap();
            let signing_key = SigningKey::from(secret_key);

            // Sign the request hash with recoverable signature
            let (signed, recovery_id) = signing_key
                .sign_prehash_recoverable(&request_hash.0)
                .unwrap();

            let mut signature = [0u8; 65];

            // Convert signature to bytes and add recovery byte
            signature[..64].copy_from_slice(&signed.to_bytes());
            signature[64] = recovery_id.to_byte();

            let origin = Origin::Secp256k1(Secp256k1Origin {
                signature: signature.into(),
                nonce: 0,
            });

            request.origin = origin;

            // println!("request_hash: {}", request_hash);
            println!("{}", serde_json::to_string(&request).unwrap());
        }
    }
}
