use std::env::args;

use commands::MergeCommand;

mod commands;

fn get_commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(MergeCommand {})]
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    let command = find_command(&args[0]).unwrap();
    command.execute(&args[1..]);
}

fn find_command(name: &str) -> Option<Box<dyn Command>> {
    get_commands().into_iter().find(|command| {
        command.get_name() == name || command.get_aliases().contains(&name.to_string())
    })
}

trait Command {
    fn get_name(&self) -> &str;

    fn get_description(&self) -> &str;

    fn get_arguments(&self) -> Vec<String>;

    fn get_subcommands(&self) -> Vec<Box<dyn Command>>;

    fn get_aliases(&self) -> &[String];

    fn execute(&self, arsgs: &[String]);
}
