use rand::Rng;

pub struct Password {
    length: usize,
}

impl Password {
    pub fn new() -> Self {
        Self::with_length(10)
    }

    pub fn with_length(length: usize) -> Self {
        Password { length }
    }
}

impl IntoIterator for Password {
    type Item = String;
    type IntoIter = PasswordsIterator;

    fn into_iter(self) -> Self::IntoIter {
        PasswordsIterator {
            length: self.length,
        }
    }
}

pub struct PasswordsIterator {
    length: usize,
}

impl Iterator for PasswordsIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::with_capacity(self.length);
        for _ in 0..self.length {
            result.push((b'a' + rand::thread_rng().gen_range(0..=(b'z' - b'a'))) as char);
        }
        Some(result)
    }
}
