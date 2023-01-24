#![allow(unused)]

use clap::Args;

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

pub(crate) struct EncodedBuffer {
  data: Vec<u16>,
}

impl EncodedBuffer {
  pub fn new() -> Self {
    Self {
      data: vec![0x5152_u16],
    }
  }

  pub fn set_width_and_height(&mut self, (width, height): (u32, u32)) {
    self.data.push(width as u16);
    self.data.push(height as u16);
  }

  pub fn get_width(&self) -> u16 {
    self.data[1]
  }

  pub fn get_height(&self) -> u16 {
    self.data[2]
  }

  pub fn get_pixel(&self, x: u16, y: u16) -> u16 {
    self.data[(y * self.get_width() + x + 2) as usize]
  }

  pub fn get_pixels(&self) -> &[u16] {
    &self.data[3..]
  }

  pub fn push(&mut self, value: u16) {
    self.data.push(value);
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn bytes(&self) -> Vec<u8> {
    self
      .data
      .iter()
      .flat_map(|two_bytes| [*two_bytes as u8, (*two_bytes >> 8) as u8])
      .collect::<Vec<u8>>()
  }

  pub fn data(&self) -> Vec<u8> {
    self
      .data
      .iter()
      .skip(3)
      .flat_map(|two_bytes| [*two_bytes as u8, (*two_bytes >> 8) as u8])
      .collect::<Vec<u8>>()
  }
}

impl From<Vec<u8>> for EncodedBuffer {
  fn from(value: Vec<u8>) -> Self {
    let mut data = value
      .chunks(2)
      .map(|two_bytes| (two_bytes[0] as u16) | ((two_bytes[1] as u16) << 8))
      .collect::<Vec<u16>>();

    if data[0] != 0x5152 {
      panic!("Invalid data");
    }

    Self { data }
  }
}

impl From<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>> for EncodedBuffer {
  fn from(value: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Self {
    let (width, height) = (value.width(), value.height());

    let mut buffer = EncodedBuffer::new();
    buffer.set_width_and_height((width, height));

    for y in 0..height {
      for x in 0..width {
        let [mut red, mut green, mut blue] = value.get_pixel(x, y).0;
        buffer.push(((red >> 3) as u16) << 11 | ((green >> 2) as u16) << 5 | ((blue >> 3) as u16));
      }
    }

    buffer
  }
}

pub(crate) fn parse_image(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
  let image = image::open(input).unwrap();
  let image = image.to_rgb8();

  let mut output_file = std::fs::File::create(output)?;

  let buffer: EncodedBuffer = image.into();

  std::io::Write::write_all(&mut output_file, &buffer.bytes())?;

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
