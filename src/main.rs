use crate::bank::Bank;
use crate::calculator::get_sum_gen;
use crate::closure::use_func;
use crate::customer::Customer;
use crate::day::Day;
use crate::guess::Guess;
use crate::math::Cacher;
use crate::math::divide;
use crate::math::largest;
use crate::shape::Circle;
use crate::shape::Point;
use crate::shape::Rectangle;
use crate::shape::Shape;
use crate::string::first_word;
use crate::string::{longest, longest_with_an_announcement};
use aggregator::{NewsArticle, Summary, Tweet};
use chrono::Local;
use std::cmp::Ordering;
// use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::thread;
use std::time::Duration;
// use std::ops::Add;
// use std::net::IpAddr;
// use std::process;
use glob::glob;

mod aggregator;
mod bank;
mod calculator;
mod closure;
mod config;
pub mod customer;
mod day;
mod file;
mod guess;
mod json;
mod math;
mod parser;
mod random;
mod shape;
mod string;
// mod traverse;

#[allow(unused)]
fn main() {
    // random::guess_random_number();
    // let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1)
    // });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // if let Err(e) = run(config) {
    //     println!("Application error : {e}");
    //     process::exit(1);
    // }

    let my_age = 18;
    let voting_age = 18;

    let current_time = Local::now();

    println!("Day is {}", current_time.date_naive());

    println!("u64 max is {}", u64::MAX);

    let factorial_result: u128 = math::factorial(30);

    println!("Factorial is {}", factorial_result);

    // println!("Random number {}", rand::thread_rng().gen_range(1..101));

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Cant vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("Wohoo you've earned the right"),
    }

    let random = String::from("Random String");
    let words = String::from("Multiple");

    let result = random + &words;
    for char in result.bytes() {
        print!("{} ", char);
    }

    println!();
    println!("Printing chars");
    let st3 = String::from("a n s i y p a");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1 {
        print!("{} ", char);
    }

    let today = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        _ => println!("Better"),
    }

    println!("Is today weekend {}", today.is_weekend());

    let _vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1, 2, 3, 4, 5];
    vec2.push(6);

    let _second = &vec2[1];

    match vec2.get(1) {
        Some(_second) => println!(),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vector 2 {:?}", vec2.pop());

    println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));

    let mut heros = HashMap::new();
    heros.insert("Superman", "Clark Kent");
    heros.insert("Batman", "Bruce Wayne");

    if heros.contains_key(&"Batman") {
        let the_batman = heros.get(&"Batman");
        match the_batman {
            Some(_) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

    for (k, v) in heros.iter() {
        println!("{} = {}", k, v);
    }

    let bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("Corldon Ave"),
        balance: 34.50,
    };

    println!("Customer is {:#?}", bob);
    println!("{}", bob.welcome());

    let _rectangle = Rectangle {
        width: 10.0,
        height: 4.2,
    };

    let rectangle: Rectangle = Shape::new(10.0, 2.3);
    let circle: Circle = Shape::new(12.2, 2.4);

    println!("The Rect is {:#?}", _rectangle);
    println!("Rect area is {}", rectangle.area());
    println!("Circle area is  {}", circle.area());

    let sum = |a: i32, b: i32| a + b;
    let prod = |a: i32, b: i32| a * b;

    println!("Sum {}", use_func(5, 4, sum));
    println!("Prod {} ", use_func(5, 8, prod));

    let mut bank = Bank::new(50.0);
    bank.withdraw(20.0);
    println!("Balance {}", bank.balance);

    let lit_x = 5;
    let some_x: Option<i32> = Some(0);

    let result_some = lit_x + some_x.unwrap_or(0);
    println!("Result of Option Addition {}", result_some);

    let hindi = "नमस्ते";
    for c in hindi.chars() {
        println!("{c}");
    }

    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map.insert(field_name, field_value);

    println!("{:?}", map);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let file = String::from("hello.txt");

    let read_from_file = file::read_username_from_file(file).unwrap();
    println!("Read username is {:?}", read_from_file);

    println!("Largest is {}", math::largest(&vec2));

    let number_list = vec![10, 20, 13, 6, 8, 100];
    let char_list = vec!['a', 'b', 'y', 'x'];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest character is {}", result);

    let p = Point { x: 10.0, y: 15.2 };

    println!("p.x = {}", p.x());
    println!("p distance = {}", p.distance_from_origin());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let string1 = String::from("Longer String");

    {
        let string2 = String::from("Short");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);

        let result_announcement =
            longest_with_an_announcement(string1.as_str(), string2.as_str(), "Important");
        println!("{}", result_announcement);
    }

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let invocation = String::from("hello world");
    let word = first_word(&invocation);

    println!("First word is {}", word);

    let mut i = 10;

    for i in [2, 1] {
        println!("{}", i);
    }

    let result = divide(2.0, 3.0);

    match result {
        Some(result) => println!("Result is {:?}", result),
        None => println!("Unable to divide by 0"),
    }

	let mut cached_result = Cacher::new(|num| {
		println!("Printing cache");
		thread::sleep(Duration::from_secs(2));
		num
	});

	cached_result.value(2);

    let guess = Guess::new(50);
    println!("{:#?}", guess);
    println!("{}", guess.hit());

    let color = RGB {
        r: 255,
        g: 80,
        b: 100,
    };
    let yuv = color.to_yuv();
    println!("YUV: ({}, {}, {})", yuv.y, yuv.u, yuv.v);

    let iast = "namo nArAyaNA";
    let slp1 = slp1_to_iast(iast);
    println!("IAST: {}", iast);
    println!("SLP1: {}", slp1);

    // parser::exif_reader();

    for entry in glob("images/*.jpg").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
    json::print_an_address().unwrap();

    let items: Vec<u64> = vec![0, 200, 1, 4];
    let mut count = 0;
    let vec = items
        .iter()
        .inspect(|_| count += 1)
        .filter(|&i| i % 2 == 0)
        .collect::<Vec<_>>();
    println!("count:{count} vec:{vec:?}");

    let hello = String::from("السلام عليكم");
    println!("{:?}", hello);

    let hello = "नमस्ते";
    for i in hello.bytes() {
        println!("{i}");
    }

    let hello = String::from("Здравствуйте");

    println!("{:?}", hello);

    let hello = String::from("안녕하세요");

    println!("{:?}", hello);
}

struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

struct YUV {
    y: f32,
    u: f32,
    v: f32,
}

impl RGB {
    fn to_yuv(&self) -> YUV {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let y = 0.299 * r + 0.587 * g + 0.114 * b;
        let u = -0.14713 * r - 0.288862 * g + 0.436 * b;
        let v = 0.615 * r - 0.51498 * g - 0.10001 * b;

        YUV { y, u, v }
    }
}

fn slp1_to_iast(input: &str) -> String {
    let map = [
        ("a", "ā"),
        ("A", "Ā"),
        ("i", "ī"),
        ("I", "Ī"),
        ("u", "ū"),
        ("U", "Ū"),
        ("f", "ḥ"),
        ("x", "ñ"),
        ("e", "ē"),
        ("E", "Ē"),
        ("o", "ō"),
        ("O", "Ō"),
    ];
    let mut transliteration_map = HashMap::new();
    for &(slp1, iast) in map.iter() {
        transliteration_map.insert(slp1, iast);
    }

    let mut result = String::new();
    for character in input.chars() {
        if let Some(iast_char) = transliteration_map.get(&character.to_string() as &str) {
            result.push_str(iast_char);
        } else {
            result.push(character);
        }
    }
    result
}
