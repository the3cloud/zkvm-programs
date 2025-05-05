use alloy_primitives::{Address, B256};
use clap::Parser;
use zktls_program_core::{Origin, Request, RequestInfo, ResponseTemplate};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

#[derive(Debug, Parser)]
enum SubCmd {
    Sign {
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

        #[clap(long = "client")]
        client: Address,

        #[clap(long = "prover-id")]
        prover_id: B256,

        #[clap(long = "secp256k1.key")]
        secp256k1_key: B256,
    },
}

fn main() {
    let args = Args::parse();

    match args.subcmd {
        SubCmd::Sign {
            request_body,
            request_addr,
            request_server,
            response_prefix,
            response_prefix_length,
            client,
            prover_id,
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

            let request = Request {
                request_info,
                response_template,
                origin: Origin::None,
                client,
                prover_id,
            };

            let request_hash = request.request_hash();

            // println!("request_hash: {}", request_hash);
            println!("{}", serde_json::to_string(&request).unwrap());
        }
    }
}
