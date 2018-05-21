#[macro_use]
extern crate lazy_static;

mod commands;
mod helpers;

use commands::add;
use commands::help;
use commands::initialize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    println!("grit v0.1");
    println!("Command: {}", command);

    match command.as_ref() {
        "init" => initialize::initialize_repo(),
        "add" => add::add(),
        _ => help::help(),
    }
}
