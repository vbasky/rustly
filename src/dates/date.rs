// use crate::dates::day::Day;
use chrono::{Datelike, Local};

pub trait Date {
    fn current_day();
}

pub struct Weekday;

impl Date for Weekday {
    fn current_day() {
        //Get the current date and time
        let now = Local::now();
        //Get the current date
        let date = now.date_naive();

        //Extract the day of the week
        let day_of_week = date.weekday().num_days_from_sunday();

        println!("Day of the week is {:?}", day_of_week);
        // println!("Is today a weekend {}", Day::is_weekend(day_of_week));
    }
}
