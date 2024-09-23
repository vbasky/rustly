use std::process;

use crate::dates::date::Date;
use crate::iterators::password::Password;
use crate::math::math::Counter;
use crate::transformers::json;
use crate::{accounting, dates, hashers, math, system, threads, transformers};

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
        println!("3: Math Largest");
        println!("4: Hashmap");
        println!("5: Accounting");
        println!("6: Date");
        println!("7: Json Serialize");
        println!("8: Mutex");
        println!("9: Print Json Address");
        println!("10: Counter");
        println!("11: Factorial");
        println!("12: Print address in memory");
        println!("13: Implement Iterators");
        println!("14: Exit");
        println!("------------------");
    }

    pub fn compute(&self) {
        match self.option {
            1 => system::info::get_system_info(),
            2 => transformers::cbor::cbor(),
            3 => {
                let value = [6, 7, 8, 9, 10];
                println!("Largest vector is {:?}", math::operations::largest(&value));
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
            6 => dates::date::Weekday::current_day(),
            7 => process::exit(0),
            8 => threads::spawn::spawn(),
            9 => json::print_an_address(),
            10 => Counter::new().for_each(|x| {
                println!("Counter value is: {}", x);
            }),
            11 => {
                let factorial_value = math::operations::factorial(120);
                println!("Factorial value is: {}", factorial_value);
            }
            12 => {
                let mut numbers = [1, 2, 3, 4, 5];
                println!("{:p}", &numbers[0]);
                for n in &mut numbers {
                    *n *= 2
                }
                for n in &numbers {
                    println!("{:p}", n);
                    println!("{}", n);
                }
            }
            13 => {
                for p in Password::new().into_iter().take(3) {
                    println!("Password is: {}", p);
                }

                Password::with_length(5)
                    .into_iter()
                    .take(3)
                    .for_each(|p| println!("The next password is {} ", p));
            }
            14 => {
                println!("Good Bye");
                process::exit(0);
            }
            _ => {
                println!("Invalid option");
                process::exit(1);
            }
        }
    }
}
