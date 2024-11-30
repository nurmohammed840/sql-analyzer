use tokenizer::token::ty;

use crate::Tokens;
pub struct Select {
    select_kw: ty::Ident,
}

impl Select {
    pub fn parse(tokens: Tokens) -> Self {
        Select {
            select_kw: todo!(),
        }
    }
}
