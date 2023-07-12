use std::collections::HashMap;

pub trait Command: Arguments {
    fn run(&self, args: Vec<String>) -> Result<(), String>;

    fn name(&self) -> &str;

    fn required_args(&self) -> Vec<String>;

    fn optional_args(&self) -> Vec<String>;
}

pub trait Arguments {
    fn set_option_args(&mut self, args: &HashMap<String, Vec<String>>) -> Result<(), String>;

    fn set_positional_args(&mut self, args: &[String]) -> Result<(), String>;
}
