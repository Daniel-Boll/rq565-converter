use clap::Parser;
use miette::{Context, Result as MietteResult};
use rq565_converter::{
  cli::{Cli, Commands},
  converter::{decode, encode},
};

fn main() -> MietteResult<()> {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Encode(encode_args) => encode::encode(encode_args).wrap_err("when encoding the file"),
    Commands::Decode(decode_args) => decode::decode(decode_args).wrap_err("when decoding the file"),
  }
}
