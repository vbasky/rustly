#[allow(dead_code)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Default for Day {
    fn default() -> Self {
        Self::Sunday
    }
}

impl Day {
    pub fn is_weekend(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => return true,
            Day::Monday | Day::Tuesday | Day::Wednesday | Day::Thursday | Day::Friday => {
                return false
            }
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
