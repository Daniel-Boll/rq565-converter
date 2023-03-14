#![allow(unused)]

use clap::Args;
use miette::Result as MietteResult;

use crate::utils::{
  constants::{EXTENSION, SUPPORTED_EXTENSIONS},
  errors::{FileFormatError, FormatError},
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

  /// The encode format to use (default: 565). Note that it should always sum up to 16 bits
  #[clap(short, long, default_value = "565")]
  format: String,
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

pub(crate) fn validate_format(format: &str) -> Result<(u8, u8, u8), FormatError> {
  let format_vec = format
    .chars()
    .map(|c| c.to_digit(10).unwrap() as u8)
    .collect::<Vec<u8>>();

  // It should be only three digits
  if format_vec.len() != 3 {
    return Err(FormatError::UnsupportedSize {
      input: format.to_string(),
      advice: "It should only be three digits".to_string(),
      extension_src: (0, format_vec.len()),
    });
  }

  // Each digit should be between 0 and 8
  for (i, digit) in format_vec.iter().enumerate() {
    if *digit > 8 {
      return Err(FormatError::UnsupportedBitValue {
        input: format.to_string(),
        advice: "Each digit should be between 0 and 8".to_string(),
        extension_src: (i, i + 1),
      });
    }
  }

  // It should sum up to 16 bits
  if format_vec.iter().sum::<u8>() != 16 {
    return Err(FormatError::UnsupportedSum {
      input: format.to_string(),
      advice: "It should sum up to 16 bits".to_string(),
      extension_src: (0, format_vec.len()),
    });
  }

  Ok((format_vec[0], format_vec[1], format_vec[2]))
}

#[derive(Clone)]
pub(crate) struct EncodedBuffer {
  pixels: Vec<u16>,
  mask: (u8, u8, u8),
  width: u16,
  height: u16,
  magic_byte: u16,
}

impl EncodedBuffer {
  pub fn new(mask: Option<(u8, u8, u8)>) -> Self {
    Self {
      magic_byte: 0x5152,
      mask: mask.unwrap_or((5, 6, 5)),
      width: 0,
      height: 0,
      pixels: Vec::new(),
    }
  }

  pub fn get_mask(&self) -> (u8, u8, u8) {
    self.mask
  }

  pub fn set_mask(&mut self, mask: (u8, u8, u8)) {
    self.mask = mask;
  }

  pub fn set_width_and_height(&mut self, (width, height): (u32, u32)) {
    self.width = width as u16;
    self.height = height as u16;
  }

  pub fn get_width(&self) -> u16 {
    self.width
  }

  pub fn get_height(&self) -> u16 {
    self.height
  }

  pub fn get_pixel(&self, x: u16, y: u16) -> u16 {
    self.pixels[(y * self.get_width() + x + 2) as usize]
  }

  pub fn get_pixels(&self) -> &[u16] {
    &self.pixels
  }

  pub fn push(&mut self, value: u16) {
    self.pixels.push(value);
  }

  pub fn len(&self) -> usize {
    self.pixels.len()
  }

  pub fn bytes(&self) -> Vec<u8> {
    let mut data: Vec<u8> = vec![
      (self.magic_byte & 0xFF) as u8,
      (self.magic_byte >> 8) as u8,
      self.mask.0,
      self.mask.1,
      self.mask.2,
      // Store width in two bytes
      (self.width & 0xFF) as u8,
      (self.width >> 8) as u8,
      // Store height in two bytes
      (self.height & 0xFF) as u8,
      (self.height >> 8) as u8,
    ];

    let pixels = self
      .pixels
      .iter()
      .flat_map(|two_bytes| [*two_bytes as u8, (*two_bytes >> 8) as u8]);
    // .flat_map(|two_bytes| [*two_bytes as u8, (*two_bytes >> 8) as u8]);

    data.extend(pixels);

    data
  }

  pub fn data(&self) -> Vec<u8> {
    self
      .pixels
      .iter()
      .flat_map(|two_bytes| [*two_bytes as u8, (*two_bytes >> 8) as u8])
      .collect::<Vec<u8>>()
  }
}

impl From<Vec<u8>> for EncodedBuffer {
  fn from(value: Vec<u8>) -> Self {
    let mut value = value;

    let magic_byte: u16 = value.remove(0) as u16 | ((value.remove(0) as u16) << 8);
    let mask = (value.remove(0), value.remove(0), value.remove(0));
    let width: u16 = value.remove(0) as u16 | ((value.remove(0) as u16) << 8);
    let height: u16 = value.remove(0) as u16 | ((value.remove(0) as u16) << 8);

    let mut data = value
      .chunks(2)
      .map(|two_bytes| (two_bytes[0] as u16) | ((two_bytes[1] as u16) << 8))
      .collect::<Vec<u16>>();

    if magic_byte != 0x5152 {
      panic!("Invalid data");
    }

    Self {
      magic_byte,
      mask,
      width,
      height,
      pixels: data,
    }
  }
}

struct ImageBufferWithMask {
  buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
  mask: (u8, u8, u8),
}

impl From<ImageBufferWithMask> for EncodedBuffer {
  fn from(value: ImageBufferWithMask) -> Self {
    let mask = value.mask;
    let image_buffer = value.buffer;
    let (width, height) = (image_buffer.width(), image_buffer.height());

    let mut buffer = EncodedBuffer::new(Some(mask));
    buffer.set_width_and_height((width, height));

    for y in 0..height {
      for x in 0..width {
        let [mut red, mut green, mut blue] = image_buffer.get_pixel(x, y).0;

        let new_red = match mask.0 {
          0 => 0,
          _ => red >> (8 - mask.0),
        };
        let new_green = match mask.1 {
          0 => 0,
          _ => green >> (8 - mask.1),
        };
        let new_blue = match mask.2 {
          0 => 0,
          _ => blue >> (8 - mask.2),
        };

        // Store the bits in the buffer as a 16 bit value
        // The first mask.0 more significant bits are red, the next mask.1 bits are green, and the last mask.2 bits are blue
        let mut store_data = 0;

        if mask.0 != 0 {
          store_data |= (new_red as u16) << (mask.1 + mask.2);
        }
        if mask.1 != 0 {
          store_data |= (new_green as u16) << mask.2;
        }
        if mask.2 != 0 {
          store_data |= new_blue as u16;
        }

        buffer.push(store_data);
      }
    }

    buffer
  }
}

pub(crate) fn parse_image(
  input: &str,
  output: &str,
  mask: (u8, u8, u8),
) -> Result<(), Box<dyn std::error::Error>> {
  let image = image::open(input).unwrap();
  let image = image.to_rgb8();

  let mut output_file = std::fs::File::create(output)?;

  let mut buffer: EncodedBuffer = ImageBufferWithMask {
    buffer: image,
    mask,
  }
  .into();

  std::io::Write::write_all(&mut output_file, &buffer.bytes())?;

  Ok(())
}

pub fn encode(
  EncodeOptions {
    input,
    output,
    format,
  }: &EncodeOptions,
) -> MietteResult<()> {
  validate_files(input, output)?;
  let mask = validate_format(format)?;
  parse_image(input, output, mask).unwrap();

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
