use std::io;

/// An error that can occur while compiling an Iron module.
#[derive(Debug)]
pub enum Error {
    /// Indicates an issue with reading from or writing to a file.
    ///
    /// This error includes the thrown `io::Error` to assist with further troubleshooting.
    IO(io::Error),

    /// Indicates that the input file is not valid ASCII.
    ///
    /// Right now, `ironc` relies on the contents of an input file being valid ASCII. In the
    /// future, better Unicode support will be added.
    UnsupportedCharacterEncoding,
}
