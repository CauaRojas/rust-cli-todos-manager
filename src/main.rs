use serde::{Deserialize, Serialize};
use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::vec;

#[derive(Serialize, Deserialize)]
struct Todo {
    desc: String,
}
fn save_state(todos: Vec<Todo>) -> std::io::Result<()> {
    let file = File::create("todos.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &todos)?;
    Ok(())
}
fn get_saved_state() -> Vec<Todo> {
    let file = match File::open("todos.json") {
        Ok(file) => file,
        Err(_) => {
            println!("Local todo file not found!\nCreating a new one...");
            return vec![];
        }
    };
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(todos) => todos,
        Err(_) => {
            println!("Local todo file not found!\nCreating a new one...");
            vec![]
        }
    }
}
fn add_todo(todos: &mut Vec<Todo>, todo: &str) {
    todos.push(Todo {
        desc: todo.to_string(),
    });
    list_todos(&todos);
}
fn list_todos(todos: &Vec<Todo>) {
    if todos.len() == 0 {
        println!("There are no todos!");
        return;
    }
    let mut index = 1;
    for todo in todos {
        println!("{} - \"{}\"", index, todo.desc);
        index += 1
    }
}
fn remove_todo(todos: &mut Vec<Todo>, index: &str) {
    let index: usize = match index.parse() {
        Ok(v) => v,
        Err(_) => {
            println!("Please provide a valid index to remove\nIndexes:");
            list_todos(&todos);
            return;
        }
    };
    if index == 0 {
        println!("Please provide a valid index to remove\nIndexes:");
        list_todos(&todos);
        return;
    }
    match todos.get(index - 1) {
        Some(_) => {
            println!("removing {}", todos.remove(index - 1).desc);
        }
        None => {
            println!("Please provide a valid index to remove\nIndexes:");
            list_todos(&todos);
            return;
        }
    }
    list_todos(&todos);
}
fn main() {
    const HELP_STRING: &str = "\
    A cli tool to manage your todos

    Usage: rust-todos [COMMAND] [VALUE]

    Command list:
    Add \"TODO TEXT\" - Adds new todo to list
    Remove NUMBER - Remove numbered todo from list (get todo number using 'list' command)
    List - Lists all todos in order
    ";

    let mut todos = get_saved_state();
    let all_args: Vec<String> = args().into_iter().collect();
    let command = all_args.get(1).unwrap_or(&String::new()).to_lowercase();
    match command.as_str() {
        "a" | "add" => match all_args.get(2) {
            Some(arg) => add_todo(&mut todos, arg),
            None => println!("Please provide a todo to be saved"),
        },
        "r" | "remove" => match all_args.get(2) {
            Some(index) => remove_todo(&mut todos, index),
            None => {
                println!("Please pass the index of the todo you want to remove\nIndexes:");
                list_todos(&todos)
            }
        },
        "l" | "list" => list_todos(&todos),
        "-h" | "help" | _ => {
            println!("{}", HELP_STRING)
        }
    }
    match save_state(todos) {
        Ok(_) => (),
        Err(_) => println!("Error while saving data, please try again!"),
    }
}
