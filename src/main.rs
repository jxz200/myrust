use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: HashMap<u32, String>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Result<TodoList, Box<dyn std::error::Error>> {
        let file = File::open("todolist.json");
        let todolist = match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)?
            },
            Err(_) => TodoList {
                tasks: HashMap::new(),
                next_id: 1,
            },
        };
        Ok(todolist)
    }

    fn add_task(&mut self, task: String) {
        self.tasks.insert(self.next_id, task.clone());
        println!("Added task {}: {}", self.next_id, task);
        self.next_id += 1;
    }

    fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.remove(&id) {
            println!("Completed task {}: {}", id, task);
        } else {
            println!("Task {} was not found", id);
        }
    }

    fn list_tasks(&self) {
        for (id, task) in &self.tasks {
            println!("{}: {}", id, task);
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create("todolist.json")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("My Todo List")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manages a todo list")
        .subcommand(SubCommand::with_name("add")
            .about("Adds a new task to the list")
            .arg(Arg::with_name("TASK")
                .help("The task to add")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("complete")
            .about("Marks a task as completed")
            .arg(Arg::with_name("ID")
                .help("The ID of the task to mark as completed")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("list")
            .about("Lists all tasks"))
        .get_matches();

    let mut todolist = TodoList::new()?;

    if let Some(matches) = matches.subcommand_matches("add") {
        let task = matches.value_of("TASK").unwrap().to_string();
        todolist.add_task(task);
    } else if let Some(matches) = matches.subcommand_matches("complete") {
        let id = matches.value_of("ID").unwrap().parse::<u32>().unwrap();
        todolist.complete_task(id);
    } else if matches.subcommand_matches("list").is_some() {
        todolist.list_tasks();
    }

    todolist.save()?;

    Ok(())
}