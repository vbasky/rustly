use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
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
            Ordering::Less => "Too small!".to_string(),
            Ordering::Greater => "Too big!".to_string(),
            Ordering::Equal => "You win!".to_string(),
        }
    }
}
