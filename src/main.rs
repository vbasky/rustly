#[allow(unused_imports)]
use parsers::config::{Config, Configuration};
// use std::{env, process};
use clap::{Parser, Subcommand};
use input::Input;
use std::process;
mod accounting;
mod algorithms;
mod dates;
mod encoders;
mod generators;
mod generics;
mod hashers;
mod input;
mod iterators;
mod math;
mod metadata;
mod parsers;
mod pointers;
mod standard;
mod system;
mod threads;
mod transformers;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    name: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Type(TypeCommand),
    Value {
        key: String,
        value: String,
        is_true: bool,
    },
}

#[derive(Debug, Clone, Parser)]
struct TypeCommand {
    value: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Container<T> {
    value: T,
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // let args = Rustly::parse();
    // let args = Arguments::parse();
    // println!("The args is {}", args.name);

    // let container = Container { value: "Something" };
    // let container2: Container<Option<&str>> = Container { value: None };

    let input = match Input::new() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = input.compute() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    //     let config = Config::build(env::args()).unwrap_or_else(|err| {
    //         println!("Didnt enter any option: {err}");
    //         process::exit(1)
    //     });
    //
    //     let result = Config::run(&self::Config {
    //         query: config.query,
    //         filename: config.filename,
    //         ignore_case: config.ignore_case,
    //     });
    //
    //     if let Err(e) = result {
    //         println!("Application error: {e}");
    //         process::exit(1);
    //     }
}
