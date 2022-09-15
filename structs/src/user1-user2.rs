struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sing_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("john@doe.com"),
        username: String::from("John Doe"),
        active: true,
        sing_in_count: 1,
    };
    
    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sing_in_count);

    user1.email = String::from("another@email.com");
    println!("{}", user1.email);

    let user2 = build_user(String::from("john2@doe.com"), String::from("John 2"));
    
    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user2.active);
    println!("{}", user2.sing_in_count);
}
