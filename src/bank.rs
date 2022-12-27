pub struct Bank {
    pub balance: f32,
}

impl Bank {
    pub fn new(balance: f32) -> Self {
        Self { balance }
    }
    pub fn withdraw(&mut self, amt: f32) {
        self.balance -= amt;
    }
}
