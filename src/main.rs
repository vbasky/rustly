#[allow(unused_imports)]
use config::{Config, Configuration};
// use std::{env, process};
use clap::{Parser, Subcommand};
use input::Input;

mod accounting;
mod algorithms;
mod args;
mod article;
mod closure;
mod config;
mod dates;
mod encoders;
mod file;
mod guess;
mod hashers;
mod input;
mod iterators;
mod math;
mod metadata;
mod mybox;
mod random;
mod shape;
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

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // let args = Rustly::parse();
    // let args = Arguments::parse();
    // println!("The args is {}", args.name);
    let input = Input::new().compute();
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
