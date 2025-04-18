// fn main() {
//     let width1 = 30;
//     let height1 = 50;
    
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1) // or just rect1 in this case
//         // usually we want to borrow the struct rather than take ownership of it
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 { // or just rectangle: Rectangle
//     // usually we want to borrow the struct rather than take ownership of it
//     rectangle.width * rectangle.height
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
//     // println!("rect1 is {}", rect1);

//     // error[E0277]: `Rectangle` doesn't implement `Debug`
//     // println!("rect1 is {rect1:?}");
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
//     println!("rect1 is {rect1:#?}"); // better for larger structs
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}