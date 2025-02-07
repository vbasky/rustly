#[allow(unused_must_use)]
#[allow(dead_code)]
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn notify(item: &impl Summary) {
        format!("Breaking news! {}", item.summarize());
    }
}

#[allow(dead_code)]
pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        format!("@{} by @{}", self.headline, self.author)
    }
}

#[allow(dead_code)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
