use core::task;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use std::env;

const FILE_PATH: &str = "data.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let iter = &args[1..];

    if iter.is_empty() {
        eprintln!("Usage: <Command> <Task>");
        return;
    }

    match iter[0].as_str() {
        "Add" => add_task(&iter[1..]),
        "Complete" => complete_task(&iter[1..]),
        "Remove" => remove_task(&iter[1..]),
        _ => eprintln!("Invalid command. Use 'Add', 'Complete', or 'Remove'."),
    }
}

//Function to read all tasks from file
fn read_tasks() -> Vec<String> {
    let file = File::open(FILE_PATH).unwrap_or_else(|_| File::create(FILE_PATH).expect("Failed to create file"));
    BufReader::new(file).lines().filter_map(Result::ok).collect()
}

//Function to write tasks back to file (after modification)
fn write_tasks(tasks: &[String]) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open file for writing");

    for task in tasks {
        writeln!(file, "{}", task).expect("Failed to write to file.");
    }
}

//Add Task
fn add_task(task_parts: &[String]) {
    if task_parts.is_empty() {
        eprintln!("Usage: Add <Task>");
        return;
    }

    let task_string = format!("{} [pending]", task_parts.join(" "));

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Failed to open file.");

    writeln!(file, "{}", task_string).expect("Failed to write to file.");

    println!("Task added!");

    display_tasks();
}

//Complete Task
fn complete_task(task_parts: &[String]) {
    if task_parts.is_empty() {
        eprintln!("Usage: Complete <Task>");
        return;
    }

    let mut tasks = read_tasks();
    let search_string = task_parts.join(" ");

    if let Some(pos) = tasks.iter().position(|task| task.starts_with(&search_string)) {
        tasks[pos] = format!("{} [complete]", search_string);
        write_tasks(&tasks);
        println!("Task marked as complete!");
    } else {
        println!("Task not found: {}", search_string);
    }

    display_tasks();
}

//Remove Task
fn remove_task(task_parts: &[String]) {
    if task_parts.is_empty() {
        eprintln!("Usage: Remove <Task>");
        return;
    }

    let mut tasks = read_tasks();
    let search_string = task_parts.join(" ");

    if let Some(pos) = tasks.iter().position(|task| task.starts_with(&search_string)) {
        println!("Removed task: {}", tasks[pos]);
        tasks.remove(pos);
        write_tasks(&tasks);
    } else {
        println!("Task not found: {}", search_string);
    }

    display_tasks();
}

//Display All Tasks
fn display_tasks() {
    let tasks = read_tasks();
    println!("\nCurrent tasks:");
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for (num, task) in tasks.iter().enumerate() {
            println!("{}. {}", num + 1, task);
        }
    }
}


//Pseudo Code Planning 

// DONE 1. I need a list of tasks from the user, empty initially. It will be a list of lists of strings. When the user inputs a task, [pending] gets automatically assigned to it.
// DONE 2. I need a function to remove the [pending] from the list element and add [complete] instead. 
// DONE 3. I need a function to remove the [pending] from the list element and add [working] instead.
// 4. I need to have some sort of clock to remove the [complete] tasks from the list the next day, and maybe some sort of reminder. 
// 5. I WANT A FRONTEND WAAAA I WANT A WAY TO STORE THE TASKS IN A DATABASE WAAAAAAAAA 

