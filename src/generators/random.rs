use rand::thread_rng;
use rand::Rng;
use std::{cmp::Ordering, io};

#[allow(dead_code)]
pub fn guess_random_number() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let random_number = thread_rng().gen::<u32>();
    println!("Another randon number is {}", random_number);

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read your input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
