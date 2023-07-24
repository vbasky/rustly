use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value should be between 1 and 100 got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn hit(&self) -> String {
        let secret_number = rand::thread_rng().gen_range(1..101);

        match self.value.cmp(&secret_number) {
            Ordering::Less => format!("Too small!"),
            Ordering::Greater => format!("Too big!"),
            Ordering::Equal => format!("You win!"),
        }
    }
}
