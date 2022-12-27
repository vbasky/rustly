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
