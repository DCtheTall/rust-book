fn main() {
    let user = User{
        email: String::from("person@example.com"),
        username: String::from("person"),
        sign_in_count: 1,
        active: true,
    };
    println!("{}", user.email);
    // Compiler error
    // user.active = false;

    // Mutability rules for struct variables apply to their fields
    let mut user = User{
        email: String::from("person@example.com"),
        username: String::from("person"),
        sign_in_count: 1,
        active: true,
    };
    user.active = false;

    let user1 = build_user(String::from("other@example.com"), String::from("fluffy"));
    // Copies other fields from user1
    let user2 = User{
        email: String::from("third@example.com"),
        username: String::from("someguy"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Function that returns a struct instance, notice the shorthand for `username` and `email`
fn build_user(username: String, email: String) -> User {
    User{
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

// Example of a tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Storing references in structs requires a lifetime parameter, covered later
// struct User2 {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }
