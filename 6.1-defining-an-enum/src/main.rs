// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(four);
//     route(six);
// }

// fn route(ip_kind: IpAddrKind) {
//     // Function implementation
//     match ip_kind {
//         IpAddrKind::V4 => println!("Routing IPv4"),
//         IpAddrKind::V6 => println!("Routing IPv6"),
//     }
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("{:?}", home);
//     println!("{:#?}", loopback);
// }

#[derive(Debug)]
enum Message {
    Write(String),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("Hello!"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // the below won't compile!
    // Option<i8> means it could be null
    // Rust doesn't have nulls
    // but it has enum to represent the concept of null
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;

    // In order to have a value that can possibly be null, 
    // you must explicitly opt in by making the type of that 
    // value Option<T>
    // You need to explicitly handle the case when the value is None
    let x: Option<i8> = Some(5);
    let y: Option<i8> = Some(5); // OR None;
    
    let sum = match (x, y) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    };
    println!("Sum: {:?}", sum);
}