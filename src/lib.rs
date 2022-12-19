#[derive(Clone, Debug)]
pub enum Token {
    INT32(i32),
    INT64(i64),
}


#[derive(Clone, Debug)]
pub struct Const {
    pub name: String,
    pub value: Token,
}

#[derive(Clone, Debug)]
pub struct Var {
    pub name: String,
    pub value: Token,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub attrs: Vec<Attr>
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub name: String,
    pub value: Token,
}
