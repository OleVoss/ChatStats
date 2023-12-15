use chrono::{Date, DateTime, Local, NaiveDate, NaiveDateTime};
use fancy_regex::Regex;
use polars::prelude::*;

pub mod analyzer;

pub struct MessageFormat {}

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    date_time: String,
    sender: String,
    content: String,
}

impl Message {
    pub fn from(datetime: &str, sender: &str, content: &str) -> Self {
        Self {
            date_time: datetime.to_string(),
            sender: sender.to_string(),
            content: content.replace("\n", "").trim().to_string(),
        }
    }
}

pub struct WhatAppChat {
    pub all_messages: Vec<Message>,
    datetime_format: String,
}

impl WhatAppChat {
    pub fn new() -> Self {
        Self {
            all_messages: vec![],
            datetime_format: "%d.%m.%YT%H:%M:%S".to_string(),
        }
    }

    pub fn from(message: &str) -> Self {
        let mut chat = WhatAppChat::new();
        chat.parse_messages(message);
        return chat;
    }

    pub fn parse_messages(&mut self, message: &str) {
        let mut messages = vec![];
        let re = Regex::new(
            r"\[(\d{2}\.\d{2}\.\d{2}), (\d{2}:\d{2}:\d{2})\] ([^:]+):((.|\n)*?)(?=\[\d{2}\.\d{2}\.\d{2}, \d{2}:\d{2}:\d{2}\] [^:]+:|\z)",
        ).unwrap();

        re.captures_iter(message).for_each(|c| {
            let reg_match = c.unwrap();
            messages.push(crate::Message::from(
                format!(
                    "{}T{}",
                    two_digit_year_fix(reg_match.get(1).unwrap().as_str()),
                    reg_match.get(2).unwrap().as_str()
                )
                .as_str(),
                reg_match.get(3).unwrap().as_str(),
                reg_match.get(4).unwrap().as_str(),
            ));
        });

        self.all_messages = messages;
    }

    pub fn get_datetime(&self) -> Vec<NaiveDateTime> {
        self.all_messages
            .iter()
            .map(|m| NaiveDateTime::parse_from_str(&m.date_time, &self.datetime_format).unwrap())
            .collect()
    }

    pub fn get_senders(&self) -> Vec<String> {
        self.all_messages.iter().map(|m| m.sender.clone()).collect()
    }

    pub fn get_contents(&self) -> Vec<String> {
        self.all_messages
            .iter()
            .map(|m| m.content.clone())
            .collect()
    }
}

fn two_digit_year_fix(date: &str) -> String {
    let mut year: String = String::from(".");
    year.push_str(&date[date.len() - 2..date.len()].to_string());
    let mut new_year = String::from(".20");
    new_year.push_str(&year[1..]);
    let new_new_year = date.replace(&year, &new_year);
    return new_new_year;
}

mod test {
    use crate::two_digit_year_fix;
    use crate::{Message, WhatAppChat};
    use chrono::NaiveDate;

    #[test]
    fn two_digit_year_fix_test() {
        let date = "13.12.23";
        assert_eq!("13.12.2023", two_digit_year_fix(date));
    }

    #[test]
    fn test_get_datetime() {
        let message = Message::from("01.01.2023T11:23:08", "testcode", "testing parsing");
        let mut chat = WhatAppChat::new();
        chat.all_messages = vec![message];

        let naivedate = NaiveDate::parse_from_str("01.01.2023T11:23:08", "%d.%m.%YT%H:%M:%S");
    }
}
