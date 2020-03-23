pub mod parser;

use parser::{Input, Parser};
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    Parser(&'a str),
}

impl<'a> From<parser::Error<'a>> for Error<'a> {
    fn from(err: parser::Error<'a>) -> Error<'a> {
        match err {
            parser::Error::ParseError(input) => Error::Parser(input),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandResult {
    Sucessfull,
    Failed,
    Effected(usize),
}

impl Default for CommandResult {
    fn default() -> Self {
        CommandResult::Sucessfull
    }
}

pub struct CommandMapping<Args, F>
where
    Args: Parser,
    F: Fn(Args) -> CommandResult + Sized,
{
    parser: PhantomData<Args>,
    command: Box<F>,
}

impl<Args, F> CommandMapping<Args, F>
where
    Args: Parser,
    F: Fn(Args) -> CommandResult + Sized,
{
    pub fn new(command: F) -> Self {
        CommandMapping {
            parser: Default::default(),
            command: Box::new(command),
        }
    }
}

pub trait Dispatcher {
    fn call<'a, C>(&self, context: &C, input: Input<'a>) -> Result<CommandResult, Error<'a>>;
}

impl<Args, F> Dispatcher for CommandMapping<Args, F>
where
    Args: Parser,
    F: Fn(Args) -> CommandResult + Sized,
{
    fn call<'a, C>(&self, context: &C, input: Input<'a>) -> Result<CommandResult, Error<'a>> {
        let (args, _) = Args::parse(context, input)?;
        let command = &self.command;
        Ok(command(args))
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{CommandMapping, Dispatcher, CommandResult, Error};
    #[test]
    fn command() {
        let command = CommandMapping::new(|_: (i32,)| {
            CommandResult::Sucessfull
        });

        assert_eq!(command.call(&(), "abc"), Err(Error::Parser("abc")));
        assert_eq!(command.call(&(), "10"), Ok(CommandResult::Sucessfull));
    }

    #[test]
    fn command_multiple_arguments() {
        let command = CommandMapping::new(|_: (i32,f32)| {
            CommandResult::Sucessfull
        });

        assert_eq!(command.call(&(), "10 4.2"), Ok(CommandResult::Sucessfull));
    }
}