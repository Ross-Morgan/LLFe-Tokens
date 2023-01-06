use crate::traits::NewData;

use super::Token;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Const {
    pub(super) name: String,
    pub(super) value: Token,
}


impl NewData for Const {
    fn new(name: String, value: Token) -> Self {
        Self { name, value }
    }
}