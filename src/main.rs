use std::env;

use maip::cli::CLI;

fn main() {
    let mut cli = CLI::build().description("maip").version("0.1.0");

    let args = get_args();

    if let Some(err) = cli.parse(args).err() {
        println!("{}", err);
    }
}

fn get_args() -> Vec<String> {
    env::args().collect::<Vec<String>>()[1..].to_vec()
}
