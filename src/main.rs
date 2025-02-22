use core::task;
use std::env;
use std::io;
fn main() {
    let args: Vec<String> = env::args().collect(); 
    let iter = &args[1..];
    let taskiter = &iter[1..];

    let mut tasks: Vec<String> = Vec::new();
    
    if iter[0] == "Add" {
        
        let task_string = taskiter.join(" ");

        tasks.push(task_string);
        println!("Task added!");


        println!("Current tasks:\n");
        
        for (num, task) in tasks.iter().enumerate() {
            println!("{num}. {task}\n");
        }

    }

    if iter[0] == "Complete" {
        

    }
}


//Pseudo Code Planning 

// 1. I need a list of tasks from the user, empty initially. It will be a list of lists of strings. When the user inputs a task, [pending] gets automatically assigned to it.
// 2. I need a function to remove the [pending] from the list element and add [complete] instead. 
// 3. I need a function to remove the [pending] from the list element and add [working] instead.
// 4. I need to have some sort of clock to remove the [complete] tasks from the list the next day, and maybe some sort of reminder. 
// 5. I WANT A FRONTEND WAAAA I WANT A WAY TO STORE THE TASKS IN A DATABASE WAAAAAAAAA 

