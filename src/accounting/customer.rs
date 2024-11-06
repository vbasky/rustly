use super::debitcard::DebitCardNumber;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Customer {
    pub name: String,
    pub address: String,
    pub balance: f32,
    pub debit_card: DebitCardNumber,
}

impl Customer {
    pub fn new(name: String, address: String, balance: f32, debit_card: DebitCardNumber) -> Self {
        Self {
            name,
            address,
            balance,
            debit_card,
        }
    }

    pub fn welcome(&self) -> String {
        String::from("Welcome ") + &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::accounting::debitcard::DebitCardNumber;

    use super::Customer;
    #[test]
    fn customer_has_proper_welcome_message() {
        let customer = Customer {
            name: String::from("Vikram"),
            address: String::from("10 Downing Street"),
            balance: 100.00,
            debit_card: DebitCardNumber {
                number: String::from("1234-5678-9012-3456"),
            },
        };
        assert_eq!(customer.welcome(), "Welcome Vikram");
    }
}
