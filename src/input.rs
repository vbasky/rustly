use std::process;

use crate::{accounting, hashers, math, system, transformers};

pub struct Input {
    pub option: u32,
}

impl Input {
    pub fn new() -> Self {
        Input::display_option();

        let input = Self::read_input("Please enter an option");
        let option = match input {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                process::exit(1);
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

    fn display_option() {
        println!("1: System Info");
        println!("2: CBOR Data");
        println!("3: Math functions");
        println!("4: Hashmap");
        println!("5: Accounting");
        println!("6: Exit");
        println!("------------------");
    }

    pub fn compute(&self) {
        match self.option {
            1 => system::info::get_system_info(),
            2 => transformers::cbor::cbor(),
            3 => {
                let value = [6, 7, 8, 9, 10];
                println!("Largest vector is {:?}", math::largest(&value));
            }
            4 => hashers::hashmap::display_hash(),
            5 => {
                let mut customer = accounting::customer::Customer::new(
                    String::from("Vikram"),
                    String::from("10 Downing Street"),
                    100.0,
                );
                println!("{}", customer.welcome());
                let mut bank = accounting::bank::Bank::new(customer.balance, &mut customer);
                bank.withdraw(50.0);
                println!("Customer balance is: {}", customer.balance);
            }
            6 => process::exit(0),
            _ => {
                println!("Invalid option");
                process::exit(1);
            }
        }
    }
}
