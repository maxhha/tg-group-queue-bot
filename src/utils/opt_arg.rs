use core::str::FromStr;
use std::convert::Infallible;
use teloxide::utils::command::ParseError;

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
        if s.len() == 0 {
            return Ok(OptArg(None));
        }

        Ok(OptArg(match T::from_str(s) {
            Ok(x) => Some(x),
            Err(_) => None,
        }))
    }
}

pub fn args_parser(s: String) -> Result<(OptArg<String>, OptArg<String>), ParseError> {
    let vec = s
        .trim()
        .split(" ")
        .skip_while(|&x| x.is_empty())
        .collect::<Vec<&str>>();

    // TODO Rewrite this part to smth more functional and readable
    match vec.len() {
        0 => return Ok((OptArg(None), OptArg(None))),
        1 => return Ok((OptArg(Some(vec[0].to_string())), OptArg(None))),
        2 => {
            return Ok((
                OptArg(Some(vec[0].to_string())),
                OptArg(Some(vec[1].to_string())),
            ))
        }
        _ => Ok((
            OptArg(Some(vec[0].to_string())),
            OptArg(Some(vec[1].to_string())),
        )),
    }
}
