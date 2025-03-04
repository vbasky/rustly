use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

pub fn print_an_address() {
    // Some data structure.
    let address = Address {
        street: String::from("10 Downing Street"),
        city: String::from("London"),
    };

    // Serialize it to a JSON string.
    let result = serde_json::to_string(&address);

    match result {
        Ok(result) => println!("Addres is {}", result),
        Err(_) => println!("Unable to parse the address"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_an_address() {
        let _result = print_an_address();
        // assert!(result);
    }
}
