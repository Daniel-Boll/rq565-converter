// Image extensions
pub const SUPPORTED_EXTENSIONS: [&str; 3] = ["png", "bmp", "ppm"];
pub const EXTENSION: &str = "rq565";

pub const LEAST_IMPORTANT_CHANNEL_MASK: u8 = 0b0001_1111;
pub const MOST_IMPORTANT_CHANNEL_MASK: u8 = 0b0011_1111;
