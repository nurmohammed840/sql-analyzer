use std::{fmt::Debug, rc::Rc};

pub mod ty {
    use std::rc::Rc;

    pub type Group = Rc<dyn super::Group>;
    pub type Ident = Rc<dyn super::Ident>;
    pub type Punct = Rc<dyn super::Punct>;
    pub type Literal = Rc<dyn super::Literal>;
}

#[derive(Clone, Debug)]
pub enum Token {
    Group(Rc<dyn Group>),
    Ident(Rc<dyn Ident>),
    Punct(Rc<dyn Punct>),
    Literal(Rc<dyn Literal>),
}

pub trait Group: Debug {
    fn tokens(&self) -> &[Token];
    fn delimiter(&self) -> Delimiter;
}

pub trait Ident: Debug {
    fn ident(&self) -> &str;
}

pub trait Punct: Debug {
    fn as_char(&self) -> char;
    fn spacing(&self) -> Spacing;
}

pub trait Literal: Debug {
    fn value(&self) -> &str;
}

#[derive(Debug, Clone, Copy)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Spacing {
    Joint,
    Alone,
}
