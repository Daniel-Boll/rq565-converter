use clap::Args;

use crate::utils::{
  constants::{EXTENSION, SUPPORTED_EXTENSIONS},
  errors::FileFormatError,
  file::{extension_src, extract_extension},
};

/// Encode an image into the RustQuant565 format
#[derive(Args)]
pub struct EncodeOptions {
  /// The input file to read
  #[clap(short, long)]
  input: String,

  /// The output file to write
  #[clap(short, long, default_value = "output.rq565")]
  output: String,
}

fn validate_files(input: &str, output: &str) -> Result<(), FileFormatError> {
  let input_extension = extract_extension(input)?;
  let output_extension = extract_extension(output)?;

  if output_extension != EXTENSION {
    return Err(FileFormatError::UnsupportedFormat {
      input: output.to_string(),
      advice: format!("It should be {}", EXTENSION),
      extension_src: extension_src(output, output_extension.len()),
    });
  }

  match input_extension {
    extension if SUPPORTED_EXTENSIONS.contains(&extension) => {}
    _ => {
      return Err(FileFormatError::UnsupportedFormat {
        input: input.to_string(),
        advice: format!("Supported extensions: {:?}", SUPPORTED_EXTENSIONS),
        extension_src: extension_src(input, input_extension.len()),
      });
    }
  }

  Ok(())
}

pub fn encode(EncodeOptions { input, output }: &EncodeOptions) -> Result<(), FileFormatError> {
  validate_files(input, output)?;

  Ok(())
}
