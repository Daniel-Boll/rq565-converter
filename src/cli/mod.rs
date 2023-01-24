use crate::converter::{decode, encode};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Encode an image into the RustQuant565 format
  Encode(encode::EncodeOptions),

  /// Decode an image from the RustQuant565 format
  Decode(decode::DecodeOptions),

  // Render(render::RenderOptions),
}
