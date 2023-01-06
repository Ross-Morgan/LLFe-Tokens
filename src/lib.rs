pub mod consts;
pub mod vars;
pub mod traits;

use consts::Const;
use vars::Var;
use traits::{Data, DataBuild, NewData};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Token {
    Int32(i32),
    Int64(i64),
    STR(String),

    VarRef(String),
    FuncRef(String),

    Register(i32),

    Attr(String, Option<Box<Self>>),

    MOV(Box<Self>, Box<Self>),
    LDR(i32, i32),

    RISC(Box<Self>),

    RUN(String),

    NOP
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
            name: self.name.expect("Function builder missing name"),
            attrs: self.attrs.expect("Function builder missing attrs"),
            contents: self.contents.expect("Function builder missing contents"),
        }
    }
}

impl Function {
    pub fn build(self) -> Self { self }
}

pub struct DataBuilder {
    name: Option<String>,
    value: Option<Token>,
}

impl Data for Const {
    fn name(&self) -> &String {
        &self.name
    }

    fn value(&self) -> &Token {
        &self.value
    }
}

impl Data for Var {
    fn name(&self) -> &String {
        &self.name
    }

    fn value(&self) -> &Token {
        &self.value
    }
}

impl DataBuilder {
    pub fn build_const(self) -> consts::Const {
        consts::Const {
            name: self.name.expect("Const builder missing name"),
            value: self.value.expect("Const builder missing value"),
        }
    }

    pub fn build_var(self) -> vars::Var {
        vars::Var {
            name: self.name.expect("Var builder missing name"),
            value: self.value.expect("Var builder missing value"),
        }
    }
}

impl DataBuild for DataBuilder {
    fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string())
    }

    fn set_value(&mut self, value: &Token) {
        self.value = Some(value.clone())
    }

    fn build<T: NewData>(self) -> T {
        T::new(
            self.name.expect("No name specified"),
            self.value.expect("No value specified"),
        )
    }
}

pub mod prelude {
    pub use super::consts::Const;
    pub use super::vars::Var;

    pub use super::Attr;
    pub use super::Function;
    pub use super::FunctionBuilder;

    pub use super::traits::Data;
    pub use super::traits::NewData;
    pub use super::Token;
}
