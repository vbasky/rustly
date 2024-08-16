// use crate::accounting::calculator::get_sum_gen;
// use crate::closure::use_func;
// use crate::day::Day;
// use crate::guess::Guess;
// use crate::math::divide;
// use crate::math::largest;
// use crate::math::Cacher;
// use crate::metadata::parser::exif_reader;
// use crate::shape::Circle;
// use crate::shape::Point;
// use crate::shape::Rectangle;
// use crate::shape::Shape;
// use crate::transformers::string::first_word;
// use crate::transformers::string::{longest, longest_with_an_announcement};

// use crate::transformers::transliterate::Transliterate;
// use article::{Article, Summary, Tweet};
// use chrono::Local;
// use config::Config;
// use config::Configure;
// use glob::glob;
use input::Input;
// use mybox::MyBox;
// use std::cmp::Ordering;
// use std::collections::HashMap;
use std::fs::File;
// use std::io::ErrorKind;
// use std::thread;
// use std::time::Duration;
// use system::info;
// use transformers::cbor::cbor;
// use transformers::string::heart;
// use transformers::transliterate::{ISlp1ToIastTransliterate, IastToSlp1Transliterate};

mod accounting;
mod article;
mod closure;
mod config;
mod day;
mod file;
mod guess;
mod hashers;
mod input;
mod math;
mod metadata;
mod mybox;
mod random;
mod shape;
mod system;
mod transformers;
// mod traverse;
// use std::ops::Add;
// use std::net::IpAddr;
//use std::process;

#[allow(unused_variables)]
fn main() {
    Input::new().compute();

    // random::guess_random_number();
    // let config = Config::build(env::args()).unwrap_or_else(|err| {
    //     println!("Didnt enter any option: {err}");
    //     process::exit(1)
    // });

    // let result = Config::run(&self::Config {
    //     query: config.query,
    //     filename: config.filename,
    //     ignore_case: config.ignore_case,
    // });

    // if let Err(e) = result {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    // println!("Rust is {:?}", heart());

    // info::get_system_info();
    // cbor();

    // let my_age = 18;
    // let voting_age = 18;
    // let age_wrapper = MyBox::new(my_age);

    // assert_eq!(18, my_age);
    // assert_eq!(18, *age_wrapper);
    // println!("{:?}", *age_wrapper);

    // let current_time = Local::now();

    // println!("Day is {}", current_time.date_naive());

    // println!("u64 max is {}", u64::MAX);

    // let factorial_result: u128 = math::factorial(30);

    // println!("Factorial is {}", factorial_result);

    // // println!("Random number {}", rand::thread_rng().gen_range(1..101));

    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("Cant vote"),
    //     Ordering::Greater => println!("Can vote"),
    //     Ordering::Equal => println!("Wohoo you've earned the right"),
    // }

    // let random = String::from("Random String");
    // let words = String::from("Multiple");

    // let result = random + &words;
    // for char in result.bytes() {
    //     print!("{} ", char);
    // }

    // println!();
    // println!("Printing chars");
    // let st3 = String::from("a n s i y p a");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();

    // for char in v1 {
    //     print!("{} ", char);
    // }

    // let today = Day::Monday;
    // match today {
    //     Day::Monday => println!("Everyone hates Monday"),
    //     _ => println!("Better"),
    // }

    // println!("Is today weekend {}", today.is_weekend());

    // let _vec1: Vec<i32> = Vec::new();
    // let mut vec2: Vec<i32> = vec![1, 2, 3, 4, 5];
    // vec2.push(6);

    // let _second = &vec2[1];

    // match vec2.get(1) {
    //     Some(_second) => println!(),
    //     None => println!("No 2nd value"),
    // }

    // for i in &mut vec2 {
    //     *i *= 2;
    // }

    // for i in &vec2 {
    //     println!("{}", i);
    // }

    // println!("Vector 2 {:?}", vec2.pop());

    // println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));

    // let bob = Customer {
    //     name: String::from("Bob Smith"),
    //     address: String::from("Corldon Ave"),
    //     balance: 34.50,
    // };

    // println!("Customer is {:#?}", bob);
    // println!("{}", bob.welcome());

    // let _rectangle = Rectangle {
    //     width: 10.0,
    //     height: 4.2,
    // };

    // let rectangle: Rectangle = Shape::new(10.0, 2.3);
    // let circle: Circle = Shape::new(12.2, 2.4);

    // println!("The Rect is {:#?}", _rectangle);
    // println!("Rect area is {}", rectangle.area());
    // println!("Circle area is  {}", circle.area());

    // let sum = |a: i32, b: i32| a + b;
    // let prod = |a: i32, b: i32| a * b;

    // println!("Sum {}", use_func(5, 4, sum));
    // println!("Prod {} ", use_func(5, 8, prod));

    // let lit_x = 5;
    // // let some_x: Option<i32> = Some(0);

    // let result_some = lit_x;
    // println!("Result of Option Addition {}", result_some);

    // let hindi = "नमस्ते";
    // for c in hindi.chars() {
    //     println!("{c}");
    // }

    // let mut map = HashMap::new();
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // map.insert(field_name, field_value);

    // println!("{:?}", map);

    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
    //     ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!("Problem creating the file: {:?}", e),
    //     },
    //     other_error => {
    //         panic!("Problem opening the file: {:?}", other_error);
    //     }
    // });

    // let file = String::from("hello.txt");

    // let read_from_file = file::read_username_from_file(file).unwrap();
    // println!("Read username is {:?}", read_from_file);

    // println!("Largest is {}", math::largest(&vec2));

    // let number_list = vec![10, 20, 13, 6, 8, 100];
    // let char_list = vec!['a', 'b', 'y', 'x'];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let result = largest(&char_list);
    // println!("The largest character is {}", result);

    // let p = Point { x: 10.0, y: 15.2 };

    // println!("p.x = {}", p.x());
    // println!("p distance = {}", p.distance_from_origin());

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probably already know, people"),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize());

    // let article = Article {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //     ),
    // };

    // println!("New article available! {}", article.summarize());

    // let string1 = String::from("Longer String");

    // {
    //     let string2 = String::from("Short");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);

    //     let result_announcement =
    //         longest_with_an_announcement(string1.as_str(), string2.as_str(), "Important");
    //     println!("{}", result_announcement);
    // }

    // struct ImportantExcerpt<'a> {
    //     part: &'a str,
    // }

    // let novel = String::from("Call me Ishmael. Some years ago...");
    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // let i = ImportantExcerpt {
    //     part: first_sentence,
    // };

    // let number: f64 = 1.0;
    // let width: usize = 5;
    // println!("{number:>width$}");

    // let invocation = String::from("hello world");
    // let word = first_word(&invocation);

    // println!("First word is {}", word);

    // let i = 10;

    // for i in [2, 1] {
    //     println!("{}", i);
    // }

    // let result = divide(2.0, 3.0);

    // match result {
    //     Some(result) => println!("Result is {:?}", result),
    //     None => println!("Unable to divide by 0"),
    // }

    // let mut cached_result = Cacher::new(|num| {
    //     println!("Printing cache");
    //     // thread::sleep(Duration::from_secs(2));
    //     num
    // });

    // cached_result.value(2);

    // let guess = Guess::new(50);
    // println!("{:#?}", guess);
    // println!("{}", guess.hit());

    // // let color = transformers::color::Rgb {
    // //     r: 255,
    // //     g: 80,
    // //     b: 100,
    // // };
    // // let yuv = color.to_yuv();
    // // println!("YUV: ({}, {}, {})", yuv.y, yuv.u, yuv.v);

    // // let iast = "namo nArAyaNA";
    // // let slp1 = slp1_to_iast(iast);
    // // println!("IAST: {}", iast);
    // // println!("SLP1: {}", slp1);

    // // parser::exif_reader();

    // for entry in glob("images/*.jpg").expect("Failed to read glob pattern") {
    //     match entry {
    //         Ok(path) => println!("{:?}", path.display()),
    //         Err(e) => println!("{:?}", e),
    //     }
    // }
    // transformers::json::print_an_address().unwrap();

    // let items: Vec<u64> = vec![0, 200, 1, 4];
    // let mut count = 0;
    // let vec = items
    //     .iter()
    //     .inspect(|_| count += 1)
    //     .filter(|&i| i % 2 == 0)
    //     .collect::<Vec<_>>();
    // println!("count:{count} vec:{vec:?}");

    // let hello = String::from("السلام عليكم");
    // println!("{:?}", hello);

    // let hello = "नमस्ते";
    // for i in hello.bytes() {
    //     println!("{i}");
    // }

    // let hello = String::from("Здравствуйте");

    // println!("{:?}", hello);

    // let hello = String::from("안녕하세요");

    // println!("{:?}", hello);

    // println!("{:?}", exif_reader());

    // let islptoiast = IastToSlp1Transliterate::new();
    // let slptoiast = ISlp1ToIastTransliterate::new();

    // let iast_text = "jñātibhirvibhajyate naiva coreṇāpi na nīyate";
    // let slp1_text = islptoiast.transliterate(iast_text);
    // let slp2_text = "Arya Sfzga fziH";
    // let iast2_text = slptoiast.transliterate(slp2_text);

    // println!("SLP1: {:?}", slp1_text);
    // println!("IAST: {:?}", iast2_text);
}
