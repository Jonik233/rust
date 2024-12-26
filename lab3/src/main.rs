use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}

struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add_task(&mut self, title: String) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, title);
        self.tasks.push(task);
        println!("Task added successfully!");
    }

    fn delete_task(&mut self, id: usize) {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            println!("Task deleted successfully!");
        } else {
            println!("Task not found!");
        }
    }

    fn edit_task(&mut self, id: usize, new_title: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.title = new_title;
            println!("Task updated successfully!");
        } else {
            println!("Task not found!");
        }
    }

    fn mark_completed(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task marked as completed!");
        } else {
            println!("Task not found!");
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for task in &self.tasks {
                println!("[{}] {} - {}", task.id, task.title, if task.completed { "Completed" } else { "Pending" });
            }
        }
    }

    fn save_to_file(&self, filename: &str) {
        let data = serde_json::to_string(&self.tasks).expect("Failed to serialize tasks.");
        fs::write(filename, data).expect("Failed to write to file.");
        println!("Tasks saved successfully!");
    }

    fn load_from_file(&mut self, filename: &str) {
        if let Ok(data) = fs::read_to_string(filename) {
            self.tasks = serde_json::from_str(&data).expect("Failed to deserialize tasks.");
            println!("Tasks loaded successfully!");
        } else {
            println!("No saved tasks found.");
        }
    }
}

fn main() {
    let mut app = TodoApp::new();
    let filename = "tasks.json";
    app.load_from_file(filename);

    loop {
        println!("\nTodo List App");
        println!("1. Add Task");
        println!("2. Delete Task");
        println!("3. Edit Task");
        println!("4. Mark Task as Completed");
        println!("5. List Tasks");
        println!("6. Save and Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                print!("Enter task title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                app.add_task(title.trim().to_string());
            }
            2 => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<usize>().unwrap_or(0);
                app.delete_task(id);
            }
            3 => {
                print!("Enter task ID to edit: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<usize>().unwrap_or(0);

                print!("Enter new task title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                app.edit_task(id, title.trim().to_string());
            }
            4 => {
                print!("Enter task ID to mark as completed: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                let id = id.trim().parse::<usize>().unwrap_or(0);
                app.mark_completed(id);
            }
            5 => {
                app.list_tasks();
            }
            6 => {
                app.save_to_file(filename);
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
