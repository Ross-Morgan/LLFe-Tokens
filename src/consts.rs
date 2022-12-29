use super::Token;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Const {
    name: String,
    value: Token,
}