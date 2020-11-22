use chrono::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};
mod email;
trait BirthdayGreeting {
    fn run(&self, file_name: String);
}

struct Mailer {}

struct Mail {
    to: String,
    from: String,
    subject: String,
    body: String,
}

impl Mail {
    fn new(first_name: &str, email_address: &str) -> Mail {
        let from = String::from("noreply@devloup.dev");
        let subject = String::from("Joyeux Anniversaire !");
        let body = format!("Bonjour {},\nJoyeux Anniversaire !\nA bientôt,", first_name);
        let to = email_address.to_string();
        Mail {
            from,
            subject,
            body,
            to,
        }
    }
}

impl Mailer {
    fn send_email(&self, mail: Mail) {
        println!("Sending email to : {}", mail.to);
        println!("subject: {}", mail.subject);
        println!("Body: {}", mail.body);
        println!("-------------------------");
    }
}

struct App();

impl BirthdayGreeting for App {
    fn run(&self, file_name: String) {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let mut first_line = true;
        println!("Reading file ...");
        for (index, line) in reader.lines().enumerate() {
            if first_line {
                first_line = false;
                continue;
            }
            let line_str = line.unwrap();
            let tokens = create_tokens(index, line_str);

            if checkdate(tokens) {
                let mail = Mail::new(&tokens[0][..], &tokens[2][..]);
                Mailer {}.send_email(mail);
            }
        }
    }
}

fn checkdate(tokens: Vec<String>) -> bool {
    let date: Vec<&str> = tokens[2].split('/').collect();
    if date.len() != 3 {
        eprintln!("Cannot read birthdate for {} {}", tokens[0], tokens[1]);
    }
    let today = Utc::now().date();
    let birthday_day = u32::from_str(date[0]).unwrap_or(0);
    let birthday_month = u32::from_str(date[1]).unwrap_or(0);

    if today.day() == birthday_day && today.month() == birthday_month {
        return true;
    }
    return false;
}

fn create_tokens(index: usize, line: String) -> Vec<String> {
    let tokens: Vec<String> = line
        .split(',')
        .map(|sub_str| sub_str.trim().to_string())
        .collect();
    if tokens.len() != 4 {
        eprintln!("Invalid file format at line {}", index);
    }
    return tokens;
}

fn main() {
    App {}.run(String::from("../employees.txt"));
}
