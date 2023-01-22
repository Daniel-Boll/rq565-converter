use clap::Args;

use crate::utils::{
  constants::{EXTENSION, SUPPORTED_EXTENSIONS},
  errors::FileFormatError,
  file::{extension_src, extract_extension},
};

/// Decode a file from the RQ565 format
#[derive(Args)]
pub struct DecodeOptions {
  /// The input file to read
  #[clap(short, long, default_value = "output.rq565")]
  input: String,

  /// The output file to write
  #[clap(short, long)]
  output: String,
}

fn validate_files(input: &str, output: &str) -> Result<(), FileFormatError> {
  let input_extension = extract_extension(input)?;
  let output_extension = extract_extension(output)?;

  if input_extension != EXTENSION {
    return Err(FileFormatError::UnsupportedFormat {
      input: input.to_string(),
      advice: format!("It should be {}", EXTENSION),
      extension_src: extension_src(input, input_extension.len()),
    });
  }

  match output_extension {
    extension if SUPPORTED_EXTENSIONS.contains(&extension) => {}
    _ => {
      return Err(FileFormatError::UnsupportedFormat {
        input: output.to_string(),
        advice: format!("Supported extensions: {:?}", SUPPORTED_EXTENSIONS),
        extension_src: extension_src(output, output_extension.len()),
      });
    }
  }

  Ok(())
}

pub fn decode(DecodeOptions { input, output }: &DecodeOptions) -> Result<(), FileFormatError> {
  validate_files(input, output)?;

  Ok(())
}
