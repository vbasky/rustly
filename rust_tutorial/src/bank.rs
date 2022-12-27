pub struct Bank {
    pub balance: f32,
}

impl Bank {
    fn new(balance: f32) -> Self {
        Self { balance }
    }
    pub fn withdraw(&mut self, amt: f32) {
        self.balance -= amt;
    }
    pub fn customer(&mut self) {
        self.withdraw(5.00);
    }
}
