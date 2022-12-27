#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Token {
    INT32(i32),
    INT64(i64),
    STR(String),

    ATTR(String, Option<Box<Self>>),

    MOV(i32, i32),
    LDR(i32, i32),

    RUN(String),
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
    pub attrs: Vec<Attr>,
    pub contents: Vec<Token>
}


#[derive(Clone, Debug, Hash, PartialEq)]
pub struct FunctionBuilder {
    pub name: Option<String>,
    pub attrs: Option<Vec<Attr>>,
    pub contents: Option<Vec<Token>>,
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
impl Eq for FunctionBuilder {}
impl Eq for Attr {}


impl FunctionBuilder {
    pub fn build(self) -> Function {
        Function {
            name: self.name.expect(""),
            attrs: self.attrs.expect(""),
            contents: self.contents.expect("")
        }
    }
}

impl Function {
    pub fn build(self) -> Self { self }
}
