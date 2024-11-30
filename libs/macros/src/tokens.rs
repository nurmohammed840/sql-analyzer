use std::rc::Rc;

use proc_macro::{Span, TokenTree};
use tokenizer::token::{self, Delimiter, Spacing, Token};

#[derive(Debug)]
pub struct Ident {
    pub ident: Box<str>,
    pub span: Span,
}

impl From<proc_macro::Ident> for Ident {
    fn from(value: proc_macro::Ident) -> Self {
        Self {
            ident: value.to_string().into(),
            span: value.span(),
        }
    }
}

impl token::Ident for Ident {
    fn ident(&self) -> &str {
        &self.ident
    }
}

#[derive(Debug)]
pub struct Punct(pub proc_macro::Punct);

impl token::Punct for Punct {
    fn as_char(&self) -> char {
        self.0.as_char()
    }
    fn spacing(&self) -> Spacing {
        match self.0.spacing() {
            proc_macro::Spacing::Joint => Spacing::Joint,
            proc_macro::Spacing::Alone => Spacing::Alone,
        }
    }
}

#[derive(Debug)]
pub struct Literal {
    pub value: Box<str>,
    pub span: Span,
}

impl From<proc_macro::Literal> for Literal {
    fn from(value: proc_macro::Literal) -> Self {
        Self {
            value: value.to_string().into(),
            span: value.span(),
        }
    }
}

impl token::Literal for Literal {
    fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug)]
pub struct Group {
    pub tokens: Box<[token::Token]>,
    pub delimiter: Delimiter,
    pub span: Span,
}

pub fn map(tt: TokenTree) -> Token {
    match tt {
        TokenTree::Group(group) => Token::Group(Rc::new(Group::from(group))),
        TokenTree::Ident(ident) => Token::Ident(Rc::new(Ident::from(ident))),
        TokenTree::Punct(punct) => Token::Punct(Rc::new(Punct(punct))),
        TokenTree::Literal(literal) => Token::Literal(Rc::new(Literal::from(literal))),
    }
}

impl From<proc_macro::Group> for Group {
    fn from(group: proc_macro::Group) -> Self {
        Group {
            tokens: group.stream().into_iter().map(map).collect(),
            delimiter: match group.delimiter() {
                proc_macro::Delimiter::Parenthesis => Delimiter::Parenthesis,
                proc_macro::Delimiter::Brace => Delimiter::Brace,
                proc_macro::Delimiter::Bracket => Delimiter::Bracket,
                proc_macro::Delimiter::None => Delimiter::None,
            },
            span: group.span(),
        }
    }
}

impl token::Group for Group {
    fn tokens(&self) -> &[token::Token] {
        &self.tokens
    }

    fn delimiter(&self) -> Delimiter {
        self.delimiter
    }
}
