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

fn main() {
    /*
     * ADD
     * REMOVE
     * LIST
     * HELP
     */
    save_state(get_saved_state()).expect("Could not save");
    for arg in args() {
        println!("{}", arg)
    }
}
