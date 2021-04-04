use core::str::FromStr;
use std::convert::Infallible;

/// Wrapper for `Option` with `FromStr` trait implementation
pub struct OptArg<T>(Option<T>);

impl<T> Into<Option<T>> for OptArg<T> {
    fn into(self) -> Option<T> {
        self.0
    }
}

impl<T: FromStr> FromStr for OptArg<T> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OptArg(match T::from_str(s) {
            Ok(x) => Some(x),
            Err(_) => None,
        }))
    }
}
