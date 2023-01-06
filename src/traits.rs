use super::Token;

pub trait Data {
    fn name(&self) -> &String;
    fn value(&self) -> &Token;
}

pub trait DataBuild {
    fn set_name(&mut self, _: &str);
    fn set_value(&mut self, _: &Token);

    fn build<T: NewData>(self) -> T;
}


pub trait NewData {
    fn new(name: String, value: Token) -> Self;
}