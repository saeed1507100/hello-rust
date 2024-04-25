struct User {
    name: String,
    email: String,
}

impl User {
    fn new(name: &str) -> Self {
        Self {
            name : name.to_string(),
            email: format!("{}@example.com", name.to_lowercase())
        }
    }
}


fn get_user_option(name: &str) -> Option<User> {
    if name == "Alice" {
        Some(User::new(name))
    } else {
        None
    }
}

fn get_user_result(name: &str) -> Result<User, String> {
    if name == "Alice" {
        Ok(User::new(name))
    } else {
        Err("No user found".to_string())
    }
}


fn main() {
    let user = User::new("Alice");
    println!("{}, {}", user.name, user.email);

    let user_option = get_user_option("Saeed");
    match user_option {
        Some(user) => println!("{}, {}", user.name, user.email),
        None => println!("No user found"),
    }

    let user_result = get_user_result("Alice");
    match user_result {
        Ok(user) => println!("{}: {}", user.name, user.email),
        Err(e) => println!("{}", e),
    }

    let user_result = get_user_result("Saeed");
    match user_result {
        Ok(user) => println!("{}: {}", user.name, user.email),
        Err(e) => println!("{}", e),
    }

}