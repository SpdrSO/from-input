use std::{
    io::{stdin, stdout, Write, Error as IoErr},
    str::FromStr,
};

use crate::error::FromInputError;

pub trait FromInput: Sized {
    type ParseErr;

    fn from_input(prompt: &str) -> Result<Self, FromInputError<Self::ParseErr>>;

    fn from_input_retry<ParseErrHandle>(
        prompt: &str,
        mut parse_err_handle: ParseErrHandle,
    ) -> Result<Self, IoErr>
    where
        ParseErrHandle: FnMut(&Self::ParseErr),
    {
        loop {
            match Self::from_input(prompt) {
                Ok(v) => return Ok(v),
                Err(e) => match e {
                    FromInputError::Io(io_err) => return Err(io_err),
                    FromInputError::Parse(parse_err) => parse_err_handle(&parse_err),
                },
            }
        }
    }
}

impl<T: FromStr> FromInput for T {
    type ParseErr = <Self as FromStr>::Err;

    fn from_input(prompt: &str) -> Result<Self, FromInputError<Self::ParseErr>> {
        write!(stdout(), "{}", prompt).map_err(FromInputError::Io)?;
        stdout()
            .flush()
            .map_err(FromInputError::Io)?;

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .map_err(FromInputError::Io)?;

        input
            .trim()
            .parse()
            .map_err(FromInputError::Parse)
    }
}
