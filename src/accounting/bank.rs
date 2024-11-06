use super::customer::Customer;

#[allow(dead_code)]
pub struct Bank<'a> {
    pub balance: f32,
    pub customer: &'a mut Customer,
}

impl<'a> Bank<'a> {
    pub fn new(balance: f32, customer: &'a mut Customer) -> Bank<'a> {
        Bank { balance, customer }
    }
    pub fn withdraw(&mut self, amt: f32) {
        self.customer.balance -= amt;
    }
}

#[cfg(test)]
mod tests {
    use crate::accounting::{customer::Customer, debitcard::DebitCardNumber};

    use super::Bank;
    #[test]
    fn bank_withdraws_properly() {
        let mut customer = Customer::new(
            String::from("Vikram"),
            String::from("10 Downing Street"),
            100.0,
            DebitCardNumber::new(String::from("1234-5678-9012-3456")),
        );
        let mut bank = Bank::new(100.0, &mut customer);
        bank.withdraw(50.0);
        assert_eq!(customer.balance, 50.0);
    }
}
