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

#[derive(Error, Diagnostic, Debug)]
pub enum FormatError {
  #[error("The mask format does not comprehend the 3 digits")]
  #[diagnostic(code(format::unsupported_size))]
  UnsupportedSize {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label("wrong size")]
    extension_src: (usize, usize),
  },

  #[error("The mask does not sum up to 16")]
  #[diagnostic(code(format::unsupported_sum))]
  UnsupportedSum {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label("the sum of each component does not equal to 16")]
    extension_src: (usize, usize),
  },

  #[error("One of the bits is not a valid value")]
  #[diagnostic(code(format::unsupported_bit_value))]
  UnsupportedBitValue {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label("Each bit should not be greater than 8")]
    extension_src: (usize, usize),
  },
}
