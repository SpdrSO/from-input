use std::io::Error as IoErr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FromInputError<ParseErr> {
    #[error("I/O error: {0}")]
    Io(IoErr),

    #[error("Parse error: {0}")]
    Parse(ParseErr),
}
