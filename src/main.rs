use std::io;
use std::process::Command;

struct Todo {
    title: String,
    completed: bool,
}

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();

    loop {
        clear_console();
        println!("Todo List App");
        println!("\n1. Add a Todo item");
        println!("2. Change Todo item status");
        println!("3. Delete a Todo item");
        println!("4. Show All Todo items");
        println!("\n0. Exit\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => add_item(&mut todo_list),
            "2" => change_status(&mut todo_list),
            "3" => delete_item(&mut todo_list),
            "4" => show_items(&todo_list),
            "0" => return,
            _ => invalid(),
        }
    }
}

fn show_items(todo_list: &Vec<Todo>) {
    clear_console();
    println!("Todo List\n");
    for (index, todo) in todo_list.iter().enumerate() {
        let status = if todo.completed { "Done" } else { "Not Done" };
        println!("{}. {} - {}", index + 1, todo.title, status);
    }
    wait_for_user();
}

fn change_status(todo_list: &mut Vec<Todo>) {
    clear_console();
    println!("Todo List\n");
    for (index, todo) in todo_list.iter().enumerate() {
        let status = if todo.completed { "Done" } else { "Not Done" };
        println!("{}. {} - {}", index + 1, todo.title, status);
    }
    println!("\nEnter the index of the Todo item to change its status:");

    let mut index_str = String::new();
    io::stdin().read_line(&mut index_str).expect("Failed to read line");
    let index = index_str.trim().parse::<usize>();

    match index {
        Ok(idx) if idx - 1 < todo_list.len() => {
            todo_list[idx - 1].completed = !todo_list[idx - 1].completed;
            println!("\nTodo item status changed.");
        }
        _ => println!("\nInvalid index. Please enter a valid index."),
    }
    wait_for_user();
}

fn add_item(todo_list: &mut Vec<Todo>) {
    clear_console();
    let mut title = String::new();
    println!("Enter Todo item title:");
    io::stdin().read_line(&mut title).expect("Failed to read line");
    let title = title.trim().to_string();

    let new_todo = Todo {
        title,
        completed: false,
    };
    todo_list.push(new_todo);

    println!("\nTodo item added successfully.");
    wait_for_user();
}

fn delete_item(todo_list: &mut Vec<Todo>) {
    clear_console();
    println!("Todo List\n");
    for (index, todo) in todo_list.iter().enumerate() {
        println!("{}. {}", index + 1, todo.title);
    }
    let mut index = String::new();
    println!("\nEnter the index of the Todo item to delete:");
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Invalid index");

    if index - 1 < todo_list.len() {
        todo_list.remove(index - 1);
        println!("\nTodo item deleted successfully.");
    } else {
        println!("\nInvalid index. No such todo item exists.");
    }
    wait_for_user();
}

fn invalid() {
    clear_console(); 
    println!("INVALID CHOICE"); 
    wait_for_user();
}

fn clear_console() {
    Command::new("cmd")
    .args(&["/c", "cls"])
    .status()
    .expect("Failed to clear console");
}

fn wait_for_user() {
    println!("\nPress Enter to continue...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
}