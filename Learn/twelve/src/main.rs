// Structs

fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("example@xyz.com"),
        username: String::from("roy"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 = {}", user1.email);

}
