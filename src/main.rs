extern crate chrono;
extern crate clap;
extern crate reqwest;
extern crate scraper;

use chrono::{offset::Local, Datelike, NaiveDate};
use clap::{Arg, Command};
use scraper::{Html, Selector};

mod months;

fn main() {
    let command = Command::new("whatday")
        .version("0.1.0")
        .author("NicKoehler")
        .about("Returns the holidays of the day scraped from https://nationaltoday.com")
        .arg(
            Arg::new("date")
                .short('d')
                .long("date")
                .value_name("DATE")
                .help("Sets the date to check in format DD/MM")
                .takes_value(true),
        )
        .get_matches();

    let date: [u32; 2] = match command.value_of("date") {
        Some(date) => {
            match NaiveDate::parse_from_str(format!("{}/2021", date).as_str(), "%d/%m/%Y") {
                Ok(date) => [date.day(), date.month()],
                Err(_) => {
                    println!("Invalid date format, please use DD/MM");
                    return;
                }
            }
        }
        _ => {
            let date = Local::today();
            [date.day(), date.month()]
        }
    };

    what_day(date[0], date[1]);
}

fn what_day(day: u32, month: u32) {
    let str_day = match day {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", day),
    };

    let month_name = months::MONTHS[month as usize - 1];
    let url = format!("https://nationaltoday.com/{}-{}-holidays", month_name, day);
    let resp = reqwest::blocking::get(url).unwrap();
    match resp.status() {
        reqwest::StatusCode::OK => {
            let parsed_html = Html::parse_document(&resp.text().unwrap());
            let selector = Selector::parse("h3.holiday-title").unwrap();
            let nodes: Vec<&str> = parsed_html
                .select(&selector)
                .map(|node| node.text().collect::<Vec<_>>()[0])
                .collect();

            println!(
                "There are {} events on {} {}:\n",
                nodes.len() as u32,
                str_day,
                month_name
            );
            for node in nodes {
                println!("• {}", node);
            }
        }
        _ => {
            println!("{}", resp.status());
        }
    }
}
