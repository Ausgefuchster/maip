use std::collections::HashMap;

pub trait Command: Arguments {
    fn run(&self, args: Vec<String>) -> Result<(), String>;

    fn name(&self) -> &str;

    fn required_args(&self) -> Vec<String>;

    fn optional_args(&self) -> Vec<String>;
}

pub trait Arguments {
    fn set_values(&mut self, args: &HashMap<String, String>) -> Result<(), String>;
}
