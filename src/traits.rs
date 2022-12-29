pub trait Data {
    fn name(&self) -> &String;
    fn value(&self) -> &Token;
}

pub trait DataBuild {
    fn set_name(&mut self, _: &str);
    fn set_value(&mut self, _: &Token);
}


impl From<