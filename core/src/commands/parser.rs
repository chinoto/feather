use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    ParseError(&'a str),
}

pub type Result<'a, T> = std::result::Result<(T, Input<'a>), Error<'a>>;
pub type Input<'a> = &'a str;

pub trait Parser: Sized {
    fn parse<'a, C>(context: &C, input: Input<'a>) -> Result<'a, Self>;
}

impl<T> Parser for (T,)
where
    T: FromStr,
{
    fn parse<'a, C>(_: &C, input: Input<'a>) -> Result<'a, Self> {
        let (head, tail) = {
            let mut split = input.splitn(2, " ");
            (split.next().unwrap_or(""), split.next().unwrap_or(""))
        };
        match head.parse() {
            Ok(value) => Ok(((value,), tail)),
            Err(_) => Err(Error::ParseError(head)),
        }
    }
}

impl<T1, T2> Parser for (T1,T2) where (T1,): Parser, (T2,): Parser {
    fn parse<'a, C>(context: &C, input: Input<'a>) -> Result<'a, Self> {
        let ((v1,), tail) = <(T1,)>::parse(context, input)?;
        let ((v2,), tail) = <(T2,)>::parse(context, tail)?;
        Ok(((v1, v2), tail))
    }    
}