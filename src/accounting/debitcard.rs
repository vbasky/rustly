use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct DebitCardNumber {
    pub number: String,
}

impl DebitCardNumber {
    pub fn new(number: String) -> Self {
        DebitCardNumber { number }
    }
}

impl Display for DebitCardNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "xxxx-xxxx-xxxx-{}", &self.number[15..19])
    }
}

#[cfg(test)]
mod tests {
    use super::DebitCardNumber;
    #[test]
    fn test_debit_card_number() {
        let debit_card = DebitCardNumber::new(String::from("1234-5678-9012-3456"));
        assert_eq!(format!("{}", debit_card), "xxxx-xxxx-xxxx-3456");
    }
}
