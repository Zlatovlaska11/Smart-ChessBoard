use colored::Colorize;

pub struct Logger;

impl Logger {
    pub fn i<T: Colorize>(&mut self, message: T) {
        println!("{} -> {}", "[INFO]".blue(), message.blue())
    }
}
