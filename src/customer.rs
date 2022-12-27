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
