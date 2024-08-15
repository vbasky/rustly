use std::process;

use crate::{math, system, transformers};

pub struct Input {
    option: u32,
}

impl Input {
    pub fn new() -> Self {
        println!("1: System Info");
        println!("2: CBOR Data");
        println!("3: Math functions");
        println!("4: Hashmap");
        println!("------------------");

        let input = Self::read_input("Please enter an option");
        let option = loop {
            match input {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input");
                    process::exit(1);
                }
            }
        };

        Input { option }
    }

    fn read_input(message: &str) -> Result<u32, std::num::ParseIntError> {
        println!("{}", message);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().parse()
    }

    pub fn compute(&self) {
        match self.option {
            1 => system::info::get_system_info(),
            2 => transformers::cbor::cbor(),
            3 => {
                let value = [6, 7, 8, 9, 10];
                println!("Largest vector is {:?}", math::largest(&value));
            }
            _ => {
                println!("Invalid option");
                process::exit(1);
            }
        }
    }
}
