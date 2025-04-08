// fn main() {
//     println!("Hello, world!");

//     // another_function(5);
//     print_labeled_measurement(5, "cm");
// }

// // fn another_function(x: i32) {
// //     println!("The value of x is: {x}");
// // }

// fn print_labeled_measurement(value: i32, unit_label: &str) {
//     println!("The measurement is: {}{}", value, unit_label);
// }

// Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far.
// // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into
// // a statement, and it will then not return a value.
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// fn five() -> i32 {
//     5
// }

fn main() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}