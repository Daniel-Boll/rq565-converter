use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum FileFormatError {
  #[error(transparent)]
  #[diagnostic(code(file_read::io_error))]
  IoError(#[from] std::io::Error),

  #[error("The file is not a supported format")]
  #[diagnostic(code(file_read::unsupported_format))]
  UnsupportedFormat {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label("wrong extension")]
    extension_src: (usize, usize),
  },
}
