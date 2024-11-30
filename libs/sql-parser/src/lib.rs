pub mod commands;
mod utils;
mod error;

use std::iter::Peekable;
use tokenizer::token::Token;

pub type Tokens<'a> = &'a mut Peekable<&'a mut (dyn Iterator<Item = Token>)>;

pub fn parse(_tokens: Tokens) {
    _tokens.peek();
}
