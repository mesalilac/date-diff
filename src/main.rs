mod args;

use args::DateDiffArgs;
use chrono::prelude::*;
use clap::Parser;
use date_component::date_component;
use std::str::FromStr;

fn main() {
    let cli = DateDiffArgs::parse();

    let start_date = cli.start_date;
    let end_date = cli.end_date;
    let split_char: char;

    if start_date.contains('-') && end_date.contains('-') {
        split_char = '-';
    } else if start_date.contains('/') && end_date.contains('/') {
        split_char = '/';
    } else {
        println!("Wrong date format");
        std::process::exit(1);
    }

    let start_date: Vec<_> = start_date.split(split_char).collect();
    let end_date: Vec<_> = end_date.split(split_char).collect();

    let start_date = Date {
        year: FromStr::from_str(start_date[0]).unwrap(),
        month: FromStr::from_str(start_date[1]).unwrap(),
        day: FromStr::from_str(start_date[2]).unwrap(),
    };

    let end_date = Date {
        year: FromStr::from_str(end_date[0]).unwrap(),
        month: FromStr::from_str(end_date[1]).unwrap(),
        day: FromStr::from_str(end_date[2]).unwrap(),
    };

    let date1 = Utc
        .ymd(
            start_date.year,
            start_date.month.try_into().unwrap(),
            start_date.day.try_into().unwrap(),
        )
        .and_hms(0, 0, 0);
    let date2 = Utc
        .ymd(
            end_date.year,
            end_date.month.try_into().unwrap(),
            end_date.day.try_into().unwrap(),
        )
        .and_hms(0, 0, 0);

    let date_diff = date_component::calculate(&date1, &date2);

    let years = date_diff.year;
    let months = date_diff.month;
    let days = date_diff.interval_day;

    let seconds = days * 86_400;
    let weeks = seconds / 604800;
    let minutes = seconds / 60;
    let hours = seconds / 3600;

    if !cli.years
        && !cli.months
        && !cli.weeks
        && !cli.days
        && !cli.hours
        && !cli.minutes
        && !cli.seconds
    {
        println!(
            "years: {}, months: {}, weeks: {}, days: {}, hours: {}, minutes: {}, seconds: {}",
            years, months, weeks, days, hours, minutes, seconds
        );
    }

    if cli.years {
        println!("{}", years);
    }
    if cli.months {
        println!("{}", months);
    }
    if cli.weeks {
        println!("{}", weeks);
    }
    if cli.days {
        println!("{}", days);
    }
    if cli.hours {
        println!("{}", hours);
    }
    if cli.minutes {
        println!("{}", minutes);
    }
    if cli.seconds {
        println!("{}", seconds);
    }
}

struct Date {
    year: i32,
    month: i32,
    day: i32,
}
