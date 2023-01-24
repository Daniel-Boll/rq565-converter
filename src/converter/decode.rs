use std::sync::Mutex;

use clap::Args;

use crate::utils::{
  constants::{
    EXTENSION, LEAST_IMPORTANT_CHANNEL_MASK, MOST_IMPORTANT_CHANNEL_MASK, SUPPORTED_EXTENSIONS,
  },
  errors::FileFormatError,
  file::{extension_src, extract_extension},
};

use super::encode::EncodedBuffer;

static AUGMENT: Mutex<bool> = Mutex::new(false);

/// Decode a file from the RQ565 format
#[derive(Args)]
pub struct DecodeOptions {
  /// The input file to read
  #[clap(short, long, default_value = "output.rq565")]
  input: String,

  /// The output file to write
  #[clap(short, long)]
  output: String,

  /// Augment the pixels back to it's full range
  #[clap(short, long, default_value = "false")]
  augment: bool,
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

pub(crate) fn get_decoded_buffer(buffer: Vec<u8>) -> Vec<u8> {
  let mut output_buffer: Vec<u8> = Vec::new();

  // Read 16 bits at a time
  for window in buffer.chunks(2) {
    let (least_important, most_important) = (window[0], window[1]);
    let current_byte = (most_important as u16) << 8 | least_important as u16;

    let red_channel_reconstructed = (current_byte >> 11) & LEAST_IMPORTANT_CHANNEL_MASK as u16;
    let green_channel_reconstructed = (current_byte >> 5) & MOST_IMPORTANT_CHANNEL_MASK as u16;
    let blue_channel_reconstructed = current_byte & LEAST_IMPORTANT_CHANNEL_MASK as u16;

    // Multiply red and blue to 2^3 and green to 2^2
    let (red_channel_reconstructed, green_channel_reconstructed, blue_channel_reconstructed) =
      if *AUGMENT.lock().unwrap() {
        (
          red_channel_reconstructed * 8,
          green_channel_reconstructed * 4,
          blue_channel_reconstructed * 8,
        )
      } else {
        (
          red_channel_reconstructed,
          green_channel_reconstructed,
          blue_channel_reconstructed,
        )
      };

    output_buffer.push(red_channel_reconstructed as u8);
    output_buffer.push(green_channel_reconstructed as u8);
    output_buffer.push(blue_channel_reconstructed as u8);
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
    &get_decoded_buffer(buffer.data()),
    width,
    height,
    image::ColorType::Rgb8,
  )?;

  Ok(())
}

pub fn decode(
  DecodeOptions {
    input,
    output,
    augment,
  }: &DecodeOptions,
) -> Result<(), FileFormatError> {
  *AUGMENT.lock().unwrap() = *augment;

  validate_files(input, output)?;
  decode_file(input, output).unwrap_or_else(|e| {
    eprintln!("Error: {e}");
  });

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::converter::encode::get_encoded_buffer;

  #[test]
  fn test_decode() {
    let ppm_raw_data = b"P3
3 3
255
255 0 0
0 255 0
0 0 255
0 0 0
255 0 255
0 0 0
1 1 1
120 120 120
255 255 255";

    // Encode
    let image = image::load_from_memory(ppm_raw_data).unwrap().to_rgb8();

    let mut output_file = std::fs::File::create("test.rq565").unwrap();
    // Save the buffer as u8 bytes
    std::io::Write::write_all(
      &mut output_file,
      &get_encoded_buffer(image)
        .iter()
        .flat_map(|two_bytes| [*two_bytes as u8, (*two_bytes >> 8) as u8])
        .collect::<Vec<u8>>(),
    )
    .unwrap();

    // Decode
    // Read the bytes of the file
    let mut input_file = std::fs::File::open("test.rq565").unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    std::io::Read::read_to_end(&mut input_file, &mut buffer).unwrap();

    // Check the bytes in this buffer
    println!("=========");
    buffer.iter().for_each(|byte| {
      println!("{byte:08b}");
    });

    let output_buffer = get_decoded_buffer(buffer);
    println!("Output buffer: {output_buffer:?}");

    // // Iterate over the buffer and print the values
    for (i, chunk) in output_buffer.chunks_exact(3).enumerate() {
      println!("Pixel {}: {} {} {}", i + 1, chunk[0], chunk[1], chunk[2]);
    }
  }
}
