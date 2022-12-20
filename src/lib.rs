#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Token {
    INT32(i32),
    INT64(i64),
    STR(String),

    ATTR(String, Option<Box<Self>>),

    MOV(Box<Self>, Box<Self>),
    LDR(Box<Self>, Box<Self>),
}


#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Const {
    pub name: String,
    pub value: Token,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Var {
    pub name: String,
    pub value: Token,
}

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct Function {
    pub name: String,
    pub attrs: Vec<Attr>
}

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct Attr {
    pub name: String,
    pub value: Option<Token>,
}


impl Eq for Token {}
impl Eq for Const {}
impl Eq for Var {}
impl Eq for Function {}
impl Eq for Attr {}
