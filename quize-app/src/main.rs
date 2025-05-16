use clap::Parser;
use serde::Deserialize;
use std::{fs, io::{self, Write}};

#[derive(Parser)]
#[command(name = "Quiz Game")]
#[command(about = "A simple terminal-based quiz game")]
struct Cli {}

#[derive(Debug, Deserialize)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
}

fn main() {
    let _cli = Cli::parse();

    let data = fs::read_to_string("questions.json").expect("Cannot read questions.json");
    let questions: Vec<Question> = serde_json::from_str(&data).expect("Invalid JSON format");

    let mut score = 0;

    println!("Welcome to the Quiz Game! \n");

    for (i, q) in questions.iter().enumerate() {
        println!("{}. {}", i + 1, q.question);
        for opt in &q.options {
            println!("{}", opt);
        }

        println!("Your answer (a / b / c / d):");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let user_answer = input.trim().to_lowercase();

        if user_answer == q.answer {
            println!("✅ Correct!\n");
            score += 1;
        } else {
            println!("❌ Wrong! Correct answer: {}\n", q.answer);
        }
    }

    println!("Quiz Complete! Your Score: {}/{}", score, questions.len());
}