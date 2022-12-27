pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    pub fn is_weekend(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => return true,
            _ => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day;

    #[test]
    fn check_is_weekend() {
        let today = Day::Monday;
        assert_eq!(today.is_weekend(), false);
    }
}
