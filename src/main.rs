// Cargo.toml  
// [dependencies]
// regex   = "1"
// colored = "2"

use std::io::{self, Write};
use regex::Regex;
use colored::Colorize;

fn main() {
    println!("{}", "RustyShield: Password Strength Tester".yellow().underline().bold());
    println!("Enter a password to check its strength:");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Error: Unable to read password.");
    let password = password.trim();

    let (strength, score) = evaluate_password(password);

    match strength {
        "Weak"     => println!("Password strength: {} (Score: {}/5)", "Weak".red(), score),
        "Moderate" => println!("Password strength: {} (Score: {}/5)", "Moderate".yellow(), score),
        "Strong"   => println!("Password strength: {} (Score: {}/5)", "Strong".green(), score),
        _          => println!("Password strength: Unknown"),
    }

    if strength != "Strong" {
        println!("{}", "Suggestions to improve your password:".blue());
        suggest_improvements(password);
    }
}

fn evaluate_password(password: &str) -> (&'static str, u8) {
    let mut score = 0;

    if password.len() >= 8 {
        score += 1;
    }
    if Regex::new("[a-z]").unwrap().is_match(password) {
        score += 1;
    }
    if Regex::new("[A-Z]").unwrap().is_match(password) {
        score += 1;
    }
    if Regex::new(r"\d").unwrap().is_match(password) {
        score += 1;
    }

    if Regex::new(r#"[!@#$%^&*(),.?"_:{}|<>]"#).unwrap().is_match(password) {
        score += 1;
    }

    let strength = match score {
        0..=2 => "Weak",
        3..=4 => "Moderate",
        5     => "Strong",
        _     => "Unknown",
    };

    (strength, score)
}

fn suggest_improvements(password: &str) {
    if password.len() < 8 {
        println!("- Make the password longer (at least 8 characters).");
    }
    if !Regex::new("[a-z]").unwrap().is_match(password) {
        println!("- Add some lowercase letters (a-z).");
    }
    if !Regex::new("[A-Z]").unwrap().is_match(password) {
        println!("- Add some uppercase letters (A-Z).");
    }
    if !Regex::new(r"\d").unwrap().is_match(password) {
        println!("- Add some digits (0-9).");
    }
    if !Regex::new(r#"[!_+=-@#$%^&*(),.?":{}|<>]"#).unwrap().is_match(password) {
        println!("- Add some special symbols (!@#$, etc.).");
    }
}
