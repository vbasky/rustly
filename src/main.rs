use crate::bank::Bank;
use crate::calculator::get_sum_gen;
use crate::closure::use_func;
use crate::customer::Customer;
use crate::day::Day;
use crate::shape::Circle;
use crate::shape::Rectangle;
use crate::shape::Shape;
use chrono::Local;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

mod bank;
mod calculator;
mod closure;
pub mod customer;
mod day;
mod factorial;
mod shape;

fn main() {
    let my_age = 18;
    let voting_age = 18;

    let current_time = Local::now();

    println!("Day is {}", current_time.date_naive());

    println!("u64 max is {}", u64::MAX);

    let factorial_result: u128 = factorial::factorial(15);

    println!("Factorial is {}", factorial_result);

    println!("Random number {}", rand::thread_rng().gen_range(1..101));

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

    println!("");
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

    println!("{}", bob.welcome());

    let _rectangle = Rectangle {
        length: 10.0,
        height: 4.2,
    };

    let rectangle: Rectangle = Shape::new(10.0, 2.3);
    let circle: Circle = Shape::new(12.2, 2.4);

    println!("Rect area is {}", rectangle.area());
    println!("Circle area is  {}", circle.area());

    let sum = |a: i32, b: i32| a + b;
    let prod = |a: i32, b: i32| a * b;

    println!("Sum {}", use_func(5, 4, sum));
    println!("Prod {} ", use_func(5, 8, prod));

    let mut bank = Bank::new(50.0);
    bank.withdraw(20.0);
    println!("Balance {}", bank.balance);
}
