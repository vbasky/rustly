pub struct Customer {
    pub name: String,
    pub address: String,
    pub balance: f32,
}

impl Customer {
    pub fn new(name: String, address: String, balance: f32) -> Self {
        Self {
            name,
            address,
            balance,
        }
    }

    pub fn welcome(&self) -> String {
        return String::from("Welcome ") + &self.name;
    }
}

#[cfg(test)]
mod tests {
    use super::Customer;
    #[test]
    fn customer_has_proper_welcome_message() {
        let customer = Customer {
            name: String::from("Vikram"),
            address: String::from("10 Downing Street"),
            balance: 100.00,
        };
        assert_eq!(customer.welcome(), "Welcome Vikram");
    }
}
