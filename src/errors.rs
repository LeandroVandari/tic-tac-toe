#[derive(Debug, PartialEq, Eq)]
pub struct InvalidPlayerChar;

#[derive(Debug, PartialEq, Eq)]
pub enum InnerBoardFromStrError {
    InvalidLength,
    InvalidChars,
}

impl From<InvalidPlayerChar> for InnerBoardFromStrError {
    fn from(_: InvalidPlayerChar) -> Self {
        Self::InvalidChars
    }
}
