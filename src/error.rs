use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    BrightnessError(brightness::Error),
    PrintError {
        explanation: String,
        source: brightness::Error,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BrightnessError(err) => write!(f, "Brightness error: {err}"),
            Error::PrintError {
                explanation,
                source,
            } => write!(f, "{explanation}. Reason: {source}"),
            // Handle other error variants here if needed
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::BrightnessError(err) => Some(err),
            Error::PrintError { source, .. } => Some(source), // Return the source of other error variants here if needed
        }
    }
}

impl From<brightness::Error> for Error {
    fn from(err: brightness::Error) -> Self {
        Error::BrightnessError(err)
    }
}
