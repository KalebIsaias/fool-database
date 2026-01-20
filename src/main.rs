use std::io::{self, Write};

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    age: u8,
    city: String,
}

struct Database {
    users: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Database { users: Vec::new() }
    }

    fn insert(&mut self, name: String, age: u8, city: String) {
        let id = self.users.len() as u32 + 1;
        let user = User { id, name, age, city };
        self.users.push(user);
        println!("User inserted with ID: {}", id);
    }

    fn select_all(&self) {
        println!("All users in the database:");
        for user in &self.users {
            println!("{:?}", user);
        }
        println!("----------------------------");
    }
}

fn main() {
    let mut db = Database::new();

    db.insert("Peter Parker".to_string(), 25, "New York".to_string());
    db.insert("Clark Kent".to_string(), 35, "Metropolis".to_string());

    println!("Welcome to fool db.");

    loop {
        println!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["select"] => {
                db.select_all();
            },
            ["insert", name, age, city] => {
                if let Ok(age) = age.parse::<u8>() {
                    db.insert(name.to_string(), age, city.to_string());
                } else {
                    println!("Invalid age: {}", age);
                }
            },
            _ => {
                println!("Unrecognized command: {}", input);
            }
        }
    }
}

