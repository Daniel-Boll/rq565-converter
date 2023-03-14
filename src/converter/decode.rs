use clap::Args;

use crate::utils::{
  constants::{EXTENSION, SUPPORTED_EXTENSIONS},
  errors::FileFormatError,
  file::{extension_src, extract_extension},
};

use super::encode::EncodedBuffer;

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
      advice: format!("It should be {EXTENSION}"),
      extension_src: extension_src(input, input_extension.len()),
    });
  }

  match output_extension {
    extension if SUPPORTED_EXTENSIONS.contains(&extension) => {}
    _ => {
      return Err(FileFormatError::UnsupportedFormat {
        input: output.to_string(),
        advice: format!("Supported extensions: {SUPPORTED_EXTENSIONS:?}"),
        extension_src: extension_src(output, output_extension.len()),
      });
    }
  }

  Ok(())
}

fn reconstruct_channel(current_byte: u16, mask: u8) -> u16 {
  match mask {
    0 => 0,
    _ => current_byte >> (16 - mask),
  }
}

// Extend the u8 type with a method upscale
trait Upscale {
  fn upscale(&self, mask: u8) -> u8;
}

impl Upscale for u16 {
  fn upscale(&self, mask: u8) -> u8 {
    ((*self & ((1 << mask) - 1)) as f32 / ((1 << mask) - 1) as f32 * 255.0) as u8
  }
}

pub(crate) fn get_decoded_buffer(buffer: EncodedBuffer) -> Vec<u8> {
  let mut output_buffer: Vec<u8> = Vec::new();

  let data = buffer.data();
  let mask = buffer.get_mask();

  println!("Decoding {mask:?} bytes");

  // Read 16 bits at a time
  for window in data.chunks(2) {
    let (least_important, most_important) = (window[0], window[1]);
    let current_byte = (most_important as u16) << 8 | least_important as u16;

    if current_byte == 0 {
      output_buffer.push(0);
      output_buffer.push(0);
      output_buffer.push(0);
      continue;
    }

    let (red_channel_reconstructed, green_channel_reconstructed, blue_channel_reconstructed) = (
      reconstruct_channel(current_byte, mask.0),
      reconstruct_channel(current_byte, mask.0 + mask.1),
      reconstruct_channel(current_byte, 16),
    );

    output_buffer.push(red_channel_reconstructed.upscale(mask.0));
    output_buffer.push(green_channel_reconstructed.upscale(mask.1));
    output_buffer.push(blue_channel_reconstructed.upscale(mask.2));
  }

  output_buffer
}

fn decode_file(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut input_file = std::fs::File::open(input)?;

  let mut buffer: Vec<u8> = Vec::new();
  std::io::Read::read_to_end(&mut input_file, &mut buffer)?;

  let buffer: EncodedBuffer = buffer.into();

  let (width, height) = (buffer.get_width() as u32, buffer.get_height() as u32);

  image::save_buffer(
    output,
    &get_decoded_buffer(buffer),
    width,
    height,
    image::ColorType::Rgb8,
  )?;

  Ok(())
}

pub fn decode(DecodeOptions { input, output }: &DecodeOptions) -> Result<(), FileFormatError> {
  validate_files(input, output)?;
  decode_file(input, output).unwrap_or_else(|e| {
    eprintln!("Error: {e}");
  });

  Ok(())
}

#[cfg(test)]
mod tests {}
