// #[derive(Debug)] // enables {:?} formatting
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn define_user() -> User {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1
// }

// fn main() {
//     let temp: User = define_user();
//     println!("{:?}", temp); // debug print
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user = build_user("example@example.com".to_string(), "bob".to_string());
//     println!("Username: {}", user.username);
// }

// fn define_user() -> User {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//         // OR
//         // email: String::from("another@example.com"),
//         // ..user1
//     };
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);

//     let Point(x, y, z) = origin;

//     println!("Origin coordinates: ({}, {}, {})", x, y, z);
// }

// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// WON'T WORK
// The compiler will complain that it needs lifetime specifiers
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }