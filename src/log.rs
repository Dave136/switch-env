use colored::*;

pub struct Log {}

impl Log {
    pub fn info(message: &str) {
        println!("{}", message.blue().bold());
    }

    pub fn error(message: &str) {
        eprintln!("{} {}", "Error!".red().bold(), message.red().bold());
    }

    pub fn success(message: &str) {
        println!("{}", message.green().bold());
    }

    pub fn status(message: &str) {
        println!("{} {}", "● Env:".green().bold(), message.normal());
    }
}
