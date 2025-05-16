use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A simple CLI To-Do list app")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Remove { index: usize },
}

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
}

const FILE_PATH: &str = "tasks.json";

fn main() {
    let cli = Cli::parse();
    let mut tasks: Vec<Task> = load_tasks();

    
    match cli.command {
        Commands::Add { task } => {
            tasks.push(Task { description: task });
            save_tasks(&tasks);
            println!("Task added.");
        }
        Commands::List => {
            for (i, task) in tasks.iter().enumerate() {
                println!("{}: {}", i, task.description);
            }
        }
        Commands::Remove { index } => {
            if index < tasks.len() {
                tasks.remove(index);
                save_tasks(&tasks);
                println!("Task removed.");
            } else {
                println!("Invalid index.");
            }
        }
    }
}

fn load_tasks() -> Vec<Task> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Unable to serialize");
    fs::write(FILE_PATH, data).expect("Unable to write file");
}
