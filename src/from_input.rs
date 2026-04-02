use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

use crate::errors::FromInputError;

pub trait FromInput: Sized {
    type ParseErr;

    fn from_input(prompt: &str) -> Result<Self, FromInputError<Self::ParseErr>>;
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
