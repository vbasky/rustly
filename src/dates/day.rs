#[allow(dead_code)]
pub enum Day {
    Sun = 1,
    Mon = 2,
    Tue = 3,
    Wed = 4,
    Thu = 5,
    Fri = 6,
    Sat = 7,
}

impl Default for Day {
    fn default() -> Self {
        Self::Sun
    }
}

#[allow(dead_code)]
impl Day {
    pub fn is_weekend(&self) -> bool {
        match self {
            Day::Sat | Day::Sun => true,
            Day::Mon | Day::Tue | Day::Wed | Day::Thu | Day::Fri => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day;

    #[test]
    fn check_is_weekend() {
        let today = Day::Mon;
        assert_eq!(today.is_weekend(), false);
    }
}
