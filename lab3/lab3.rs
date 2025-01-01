use std::collections::HashMap;
use std::fs::{File};
use std::io::{self, BufRead, BufReader, Write};


fn main() {
    let mut app = ToDoApp::new();

    app.load_users("users.txt");
    app.load_tasks("tasks.txt");

    app.login_or_register();

    loop {
        println!("\n1. Add Task");
        println!("2. Delete Task");
        println!("3. Edit Task (description)");
        println!("4. Mark Task as Done");
        println!("5. List Tasks");
        println!("6. Save & Exit");
        println!("Enter your choice:");

        match read_line().as_str() {
            "1" => {
                println!("Enter task description:");
                let description = read_line();
                app.add_task(description);
            }
            "2" => {
                println!("Enter task ID to delete:");
                if let Ok(task_id) = read_line().parse() {
                    app.delete_task(task_id);
                } else {
                    println!("Invalid task ID.");
                }
            }
            "3" => {
                println!("Enter task ID to edit:");
                if let Ok(task_id) = read_line().parse() {
                    println!("Enter new description:");
                    let description = read_line();
                    app.edit_task(task_id, description);
                } else {
                    println!("Invalid task ID.");
                }
            }
            "4" => {
                println!("Enter task ID to mark as done:");
                if let Ok(task_id) = read_line().parse() {
                    app.mark_task_done(task_id);
                } else {
                    println!("Invalid task ID.");
                }
            }
            "5" => app.list_tasks(),
            "6" => {
                app.save_users("users.txt");
                app.save_tasks("tasks.txt");
                println!("Data saved. Exiting.");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}


//Структура для роботи з завданнями
#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

//Структура для роботи з самою програмою
struct ToDoApp {
    users: HashMap<String, String>,
    tasks: HashMap<String, Vec<Task>>,
    next_id: usize,
    current_user: String,
}

impl ToDoApp {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            tasks: HashMap::new(),
            next_id: 1,
            current_user: String::new(),
        }
    }
    // Передавання користувачів в HashMap 
    fn load_users(&mut self, filename: &str) {
        if let Ok(file) = File::open(filename) {
            for line in BufReader::new(file).lines() {
                if let Ok(entry) = line {
                    let parts: Vec<&str> = entry.split(':').collect();
                    if parts.len() == 2 {
                        self.users.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
            }
        }
    }

    // Збереження користувачів
    fn save_users(&self, filename: &str) {
        if let Ok(mut file) = File::create(filename) {
            for (username, password) in &self.users {
                let _ = writeln!(file, "{}:{}", username, password);
            }
        }
    }

    // Передавання завдань в HashMap 
    fn load_tasks(&mut self, filename: &str) {
        if let Ok(file) = File::open(filename) {
            for line in BufReader::new(file).lines() {
                if let Ok(entry) = line {
                    let parts: Vec<&str> = entry.split('|').collect();
                    if parts.len() == 4 {
                        let username = parts[0].to_string();
                        let task_id = parts[1].parse::<usize>().unwrap_or(0);
                        let description = parts[2].to_string();
                        let done = parts[3].parse::<bool>().unwrap_or(false);

                        let task = Task {
                            id: task_id,
                            description,
                            done,
                        };

                        self.tasks.entry(username).or_default().push(task.clone());
                        self.next_id = self.next_id.max(task_id + 1);
                    }
                }
            }
        }
    }

    // Збереження завдань
    fn save_tasks(&self, filename: &str) {
        if let Ok(mut file) = File::create(filename) {
            for (username, user_tasks) in &self.tasks {
                for task in user_tasks {
                    let _ = writeln!(
                        file,
                        "{}|{}|{}|{}",
                        username, task.id, task.description, task.done
                    );
                }
            }
        }
    }

    // Перевірка на те чи містять текстові дані : або |, позаяк вони використовуються для розділення данних при читанні файлів
    fn is_valid_input(input: &str) -> bool {
        !(input.contains(':') || input.contains('|'))
    }
    
    // Реєстрація, логін користувача
    fn login_or_register(&mut self) {
        println!("Login or Register.");
        println!("Enter username:");
        let mut username = read_line();
        loop {
            if Self::is_valid_input(&username) {
                break;
            } else {
                println!("Username cannot contain ':' or '|'.");
                println!("Enter username:");
                username = read_line();
            }
        }

        if self.users.contains_key(&username) {
            println!("Enter password:");
            let password = read_line();
            if self.users.get(&username) == Some(&password) {
                println!("Logged in as {}", username);
                self.current_user = username;
            } else {
                println!("Incorrect password.");
            }
        } else {
            println!("Registration.");
            println!("Enter password:");
            let password = read_line();

            if !Self::is_valid_input(&password) {
                println!("Password cannot contain ':' or '|'.");
                return;
            }
            self.users.insert(username.clone(), password);
            self.tasks.insert(username.clone(), Vec::new());
            println!("Registered and logged in as {}", username);
            self.current_user = username;
        }
    }

    // Додавання нового завдання
    fn add_task(&mut self, description: String) {
        if !Self::is_valid_input(&description) {
            println!("Task description cannot contain ':' or '|'.");
            return;
        }
        let task = Task {
            id: self.next_id,
            description,
            done: false,
        };
        self.tasks.entry(self.current_user.clone()).or_default().push(task);
        self.next_id += 1;
        println!("Task added.");
    }

    // Видалення існуючого завдання
    fn delete_task(&mut self, task_id: usize) {
        if let Some(user_tasks) = self.tasks.get_mut(&self.current_user) {
            if let Some(pos) = user_tasks.iter().position(|t| t.id == task_id) {
                user_tasks.remove(pos);
                println!("Task deleted.");
            } else {
                println!("Task not found.");
            }
        }
    }

    // Редагування існуючого завдання
    fn edit_task(&mut self, task_id: usize, new_description: String) {
        if !Self::is_valid_input(&new_description) {
            println!("Task description cannot contain ':' or '|'.");
            return;
        }
        if let Some(user_tasks) = self.tasks.get_mut(&self.current_user) {
            if let Some(task) = user_tasks.iter_mut().find(|t| t.id == task_id) {
                task.description = new_description;
                println!("Task updated.");
            } else {
                println!("Task not found.");
            }
        }
    }

    // Позначення існуючого завдання як виконаного
    fn mark_task_done(&mut self, task_id: usize) {
        if let Some(user_tasks) = self.tasks.get_mut(&self.current_user) {
            if let Some(task) = user_tasks.iter_mut().find(|t| t.id == task_id) {
                task.done = true;
                println!("Task marked as done.");
            } else {
                println!("Task not found.");
            }
        }
    }

    // Виведення завдань користувача
    fn list_tasks(&self) {
        if let Some(user_tasks) = self.tasks.get(&self.current_user) {
            if user_tasks.is_empty() {
                println!("You have no tasks.");
            } else {
                println!("Your Tasks:");
                for task in user_tasks {
                    println!(
                        "[{}] {} - [{}]",
                        task.id,
                        task.description,
                        if task.done { "Done" } else { "In progress" }
                    );
                }
            }
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().to_string()
}

