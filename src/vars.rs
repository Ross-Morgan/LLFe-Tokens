use super::Token;


#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Var {
    name: String,
    value: Token,
}