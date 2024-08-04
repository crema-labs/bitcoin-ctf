// utils

use colored::Colorize;

pub fn print_success_messege() {
    println!("{}", "===========================================".green());
    println!(
        "{}",
        "🎉 Congratulations, Master Hacker! 🎉"
            .bright_green()
            .bold()
    );
    println!("{}", "===========================================".green());
}

pub fn print_failure_messege() {
    println!("{}", "===========================================".red());
    println!("{}", "😢 Oh no! Time's up, hacker! 😢".bright_red().bold());
    println!("{}", "===========================================".red());
}
