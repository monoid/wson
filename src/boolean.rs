use nom::{bytes::complete::tag, combinator::value, IResult};

use crate::WsonError;

pub fn true_parser<'inp, E: WsonError<'inp>>(input: &'inp str) -> IResult<&'inp str, bool, E> {
    value(true, tag("true"))(input)
}

pub fn false_parser<'inp, E: WsonError<'inp>>(input: &'inp str) -> IResult<&'inp str, bool, E> {
    value(false, tag("false"))(input)
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    #[test]
    fn pass_true() {
        assert_eq!(true_parser::<()>("true false"), Ok((" false", true)));
    }

    #[test]
    fn failed_true() {
        assert_eq!(
            true_parser::<nom::error::Error<&str>>("false"),
            Err(Err::Error(Error::new("false", ErrorKind::Tag)))
        );
    }

    #[test]
    fn pass_false() {
        assert_eq!(false_parser::<()>("false true"), Ok((" true", false)));
    }

    #[test]
    fn failed_false() {
        assert_eq!(
            false_parser::<nom::error::Error<&str>>("true"),
            Err(Err::Error(Error::new("true", ErrorKind::Tag)))
        );
    }
}
