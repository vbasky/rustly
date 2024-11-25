#[allow(dead_code)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Day {
    Sunday = 1,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Default for Day {
    fn default() -> Self {
        Self::Sunday
    }
}

#[allow(dead_code)]
impl Day {
    // Get day name as string
    pub fn name(self) -> &'static str {
        match self {
            Day::Sunday => "Sunday",
            Day::Monday => "Monday",
            Day::Tuesday => "Tuesday",
            Day::Wednesday => "Wednesday",
            Day::Thursday => "Thursday",
            Day::Friday => "Friday",
            Day::Saturday => "Saturday",
        }
    }

    pub fn next_day(self) -> Self {
        match self {
            Day::Sunday => Day::Monday,
            Day::Monday => Day::Tuesday,
            Day::Tuesday => Day::Wednesday,
            Day::Wednesday => Day::Thursday,
            Day::Thursday => Day::Friday,
            Day::Friday => Day::Saturday,
            Day::Saturday => Day::Sunday,
        }
    }
}

#[allow(dead_code)]
impl Day {
    // Check if it's a weekend
    pub fn is_weekend(self) -> bool {
        matches!(self, Day::Saturday | Day::Sunday)
    }

    // Check if it's a weekday
    pub fn is_weekday(self) -> bool {
        !self.is_weekend()
    }
}

#[cfg(test)]
mod tests {
    use super::Day;

    #[test]
    fn test_is_weekend() {
        assert!(Day::Sunday.is_weekend());
        assert!(Day::Saturday.is_weekend());
        assert!(!Day::Monday.is_weekend());
    }

    #[test]
    fn test_is_weekday() {
        assert!(Day::Monday.is_weekday());
        assert!(!Day::Sunday.is_weekday());
    }

    #[test]
    fn test_next_day() {}

    #[test]
    fn test_day_name() {
        assert_eq!(Day::Monday.name(), "Monday");
        assert_eq!(Day::Sunday.name(), "Sunday");
    }
}
