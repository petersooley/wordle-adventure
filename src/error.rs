use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    WordTooSmall,
    WordTooBig,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WordTooBig => write!(f, "too many characters"),
            Self::WordTooSmall => write!(f, "too few characters"),
        }
    }
}
