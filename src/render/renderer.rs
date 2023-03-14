use clap::Args;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::ttf::Font;

use crate::converter::decode::get_decoded_buffer;
use crate::converter::encode::EncodedBuffer;
use crate::utils::constants::EXTENSION;
use crate::utils::errors::FileFormatError;
use crate::utils::file::{extension_src, extract_extension};

/// Render an image from the RustQuant565 format
#[derive(Args)]
pub struct RendererOptions {
  /// The input file to read
  #[clap(short, long)]
  input: String,
}

pub(crate) fn validate_files(input: &str) -> Result<(), FileFormatError> {
  let input_extension = extract_extension(input)?;

  if input_extension != EXTENSION {
    return Err(FileFormatError::UnsupportedFormat {
      input: input.to_string(),
      advice: format!("It should be {EXTENSION}"),
      extension_src: extension_src(input, input_extension.len()),
    });
  }

  Ok(())
}

pub fn render(RendererOptions { input }: &RendererOptions) -> Result<(), FileFormatError> {
  validate_files(input)?;
  let mut input_file = std::fs::File::open(input)?;
  let file_size = input_file.metadata()?.len();

  let mut buffer: Vec<u8> = Vec::new();
  std::io::Read::read_to_end(&mut input_file, &mut buffer)?;

  let buffer: EncodedBuffer = buffer.into();

  // Initialize SDL2
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();
  let ttf_context = sdl2::ttf::init().unwrap();

  let font = ttf_context.load_font("assets/font.ttf", 10).unwrap();

  // Create a window
  let window = video_subsystem
    .window(
      "RustQuant565 Renderer",
      buffer.get_width().into(),
      buffer.get_height().into(),
    )
    .position_centered()
    .build()
    .unwrap();

  // Get a reference to the window's canvas
  let mut canvas = window.into_canvas().build().unwrap();
  draw_image(&mut canvas, &mut event_pump, &font, buffer, file_size);

  Ok(())
}

pub(crate) fn draw_image(
  canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
  event_pump: &mut sdl2::EventPump,
  font: &Font,
  buffer: EncodedBuffer,
  file_size: u64,
) {
  let image = get_decoded_buffer(buffer.clone());
  let mut running = true;
  let mut show_size = false;
  while running {
    // Handle events
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. }
        | sdl2::event::Event::KeyDown {
          keycode: Some(sdl2::keyboard::Keycode::Escape),
          ..
        } => {
          running = false;
        }
        sdl2::event::Event::KeyDown {
          keycode: Some(sdl2::keyboard::Keycode::T),
          ..
        } => {
          show_size = !show_size;
        }
        _ => {}
      }
    }

    // Draw the image
    canvas.clear();

    let mut x = 0;
    let mut y = 0;

    // 3 bytes per pixel
    for pixel in image.chunks(3) {
      let color = Color::RGB(pixel[0], pixel[1], pixel[2]);
      canvas.set_draw_color(color);
      canvas.draw_point(Point::new(x, y)).unwrap();
      x += 1;
      if x == buffer.get_width().into() {
        x = 0;
        y += 1;
      }
    }

    // Create a surface from the text
    let surface = font
      .render(&format!("{file_size}B"))
      .solid(Color::RGB(255, 255, 255))
      .unwrap();

    // Create a texture from the surface
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .unwrap();

    // Copy the texture to the canvas in the middle of the screen
    if show_size {
      canvas
        .copy(
          &texture,
          sdl2::rect::Rect::new(0, 0, 100, 100),
          sdl2::rect::Rect::new(
            0,
            0,
            (buffer.get_width()) as u32,
            (buffer.get_height() / 4) as u32,
          ),
        )
        .unwrap();
    }

    canvas.present();
  }
}

// pub(crate) fn draw_image(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, image: &Vec<u16>) {
//   let mut x = 0;
//   let mut y = 0;
//   for pixel in image {
//     let color = Color::RGB(
//       ((pixel & 0b1111100000000000) >> 11) as u8,
//       ((pixel & 0b0000011111100000) >> 5) as u8,
//       (pixel & 0b0000000000011111) as u8,
//     );
//     canvas.set_draw_color(color);
//     canvas.draw_point(Point::new(x, y)).unwrap();
//     x += 1;
//     if x == 800 {
//       x = 0;
//       y += 1;
//     }
//   }
// }
