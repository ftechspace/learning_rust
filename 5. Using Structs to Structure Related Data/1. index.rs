
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {

    let mut user1 = User {
        email: String::from("fortune@gmail.com"),
        username: String::from("fortune11"),
        active: true,
        sign_in_count: 1,
    };
    let user2_email = String::from("user2@gmail.com");
    let user2_username = String::from("user2name");
    let user2 = create_user(user2_email, user2_username);
    // change values 
    user1.email = String::from("fortunetede1@example.com");
    println!("user1 {} {} {} {}", user1.email, user1.username, user1.active, user1.sign_in_count);
    println!("user2 {} {} {} {}", user2.email, user2.username, user2.active, user2.sign_in_count);
}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}