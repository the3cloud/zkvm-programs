use clap::Parser;

mod sign;

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

#[derive(Debug, Parser)]
enum SubCmd {
    Sign(sign::Sign),
}

fn main() {
    let args = Args::parse();

    match args.subcmd {
        SubCmd::Sign(sign) => sign.run().unwrap(),
    }
}
