use clap::Args;

/// Render an image from the RustQuant565 format
#[derive(Args)]
pub struct RendererOptions {
  /// The input file to read
  #[clap(short, long)]
  input: String,
}
