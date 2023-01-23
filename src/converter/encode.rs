use clap::Args;
use image::ImageBuffer;

use crate::utils::{
  constants::{
    EXTENSION, LEAST_IMPORTANT_CHANNEL_MASK, MOST_IMPORTANT_CHANNEL_MASK, SUPPORTED_EXTENSIONS,
  },
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

pub(crate) fn validate_files(input: &str, output: &str) -> Result<(), FileFormatError> {
  let input_extension = extract_extension(input)?;
  let output_extension = extract_extension(output)?;

  if output_extension != EXTENSION {
    return Err(FileFormatError::UnsupportedFormat {
      input: output.to_string(),
      advice: format!("It should be {EXTENSION}"),
      extension_src: extension_src(output, output_extension.len()),
    });
  }

  match input_extension {
    extension if SUPPORTED_EXTENSIONS.contains(&extension) => {}
    _ => {
      return Err(FileFormatError::UnsupportedFormat {
        input: input.to_string(),
        advice: format!("Supported extensions: {SUPPORTED_EXTENSIONS:?}"),
        extension_src: extension_src(input, input_extension.len()),
      });
    }
  }

  Ok(())
}

pub(crate) fn get_encoded_buffer(image: ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Vec<u16> {
  let (width, height) = (image.width(), image.height());

  let mut buffer: Vec<u16> = Vec::new();
  for y in 0..height {
    for x in 0..width {
      let [mut red, mut green, mut blue] = image.get_pixel(x, y).0;

      red &= LEAST_IMPORTANT_CHANNEL_MASK;
      green &= MOST_IMPORTANT_CHANNEL_MASK;
      blue &= LEAST_IMPORTANT_CHANNEL_MASK;

      buffer.push((red as u16) << 11 | (green as u16) << 5 | blue as u16);
    }
  }

  buffer
}

pub(crate) fn parse_image(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
  let image = image::open(input).unwrap();
  let image = image.to_rgb8();

  let mut output_file = std::fs::File::create(output)?;

  std::io::Write::write_all(
    &mut output_file,
    &get_encoded_buffer(image)
      .iter()
      .flat_map(|two_bytes| [*two_bytes as u8, (*two_bytes >> 8) as u8])
      .collect::<Vec<u8>>(),
  )?;

  Ok(())
}

pub fn encode(EncodeOptions { input, output }: &EncodeOptions) -> Result<(), FileFormatError> {
  validate_files(input, output)?;
  parse_image(input, output).unwrap();

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_files() {
    let input = "input.png";
    let output = "output.rq565";

    assert!(validate_files(input, output).is_ok());
  }

  #[test]
  fn test_validate_files_invalid_input() {
    let input = "input.bmp";
    let output = "output.rq565";

    assert!(validate_files(input, output).is_ok());
  }

  #[test]
  fn test_validate_files_invalid_output() {
    let input = "input.png";
    let output = "output.bmp";

    assert!(validate_files(input, output).is_err());
  }
}
