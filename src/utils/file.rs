/// Extract the extension from a file path
pub fn extract_extension(path: &str) -> Result<&str, std::io::Error> {
  let path = std::path::Path::new(path);
  path
    .extension()
    .and_then(std::ffi::OsStr::to_str)
    .ok_or_else(|| {
      std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "The provided path does not have an extension",
      )
    })
}

/// Return the begin index of the extension and the size of it
pub fn extension_src(filename: &str, extension_size: usize) -> (usize, usize) {
  (filename.find('.').unwrap() + 1, extension_size)
}
