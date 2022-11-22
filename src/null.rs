use nom::{bytes::complete::tag, combinator::value, IResult};

use crate::WsonError;

/// Recognize null
///
/// ```rust
/// use nom::error::{ErrorKind, Error};
/// use nom::Err;
/// use wson::null::{null, Null};
/// # fn main() {
///
///
/// // the parser will parse "null"
/// assert_eq!(null::<Error<&str>>("null"), Ok(("", Null)));
///
/// // this will fail
/// assert_eq!(null::<Error<&str>>("a"), Err(Err::Error(Error::new("a", ErrorKind::Tag))));
/// # }
/// ```
// null = "null"
pub fn null<'inp, E: WsonError<'inp>>(input: &'inp str) -> IResult<&'inp str, Null, E> {
    value(Null, tag("null"))(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Null;
