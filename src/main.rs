mod todo_app;

use std::io;
use std::io::Write;
use todo_app::{TodoApp};

fn main() {
    let mut app = TodoApp::new();

    loop {
        app.list(&mut io::stdout());
        println!();
        println!("Welcome to your Todo app");
        usage();
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.starts_with("add") {
            let text = input.strip_prefix("add").unwrap().trim().to_string();
            app.add(text);
            println!("Todo item added successfully!");
        } else if input.starts_with("complete") {
            let id = input.strip_prefix("complete").unwrap().trim().parse::<usize>().unwrap();
            app.complete(id);
            println!("Todo item marked as completed!");
        } else if input == "list" {
            // Do nothing, as the list is already displayed at the beginning of each iteration
        } else if input == "help" {
            usage();
        } else if input == "x" {
            println!("Exiting...");
            break;
        } else {
            println!("Invalid command. Type '?' to see available commands.");
        }

        println!();
    }
}

fn usage() {
    println!("Available commands:");
    println!("  add <todo_text>    - Add a new todo item");
    println!("  complete <todo_id> - Mark a todo item as completed");
    println!("  list               - List all todo items");
    println!("  ?                  - Show available commands");
    println!("  x                  - Exit the program");
}
