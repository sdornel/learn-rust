fn main() {
    // temperature_conversion(29, 'C');
    // println!("{}", generate_nth_fibonacci_nnumber(10));
    twelve_days_of_christmas();
}

// fn temperature_conversion(temp: i32, type_of_temp: char) {
//     if type_of_temp == 'F' {
//         let celsius = (temp - 32) * 5 / 9;
//         println!("The temperature in Celsius is {celsius}");
//     } else if type_of_temp == 'C' {
//         let fahrenheit = (temp * 9 / 5) + 32;
//         println!("The temperature in Fahrenheit is {fahrenheit}");
//     } else {
//         println!("Invalid type of temperature. Use F or C");
//     }
// }

// fn generate_nth_fibonacci_nnumber(n: i32) -> i32 {
//     if n <= 1 {
//         return n;
//     }
//     generate_nth_fibonacci_nnumber(n - 1) + generate_nth_fibonacci_nnumber(n - 2)
// }

fn twelve_days_of_christmas() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me:", days[i]);
        for j in (0..=i).rev() {
            if i == 0 && j == 0 {
                println!("{}", gifts[j]);
            } else if j == 0 {
                println!("and {}", gifts[j]);
            }
            else {
                println!("{}", gifts[j]);
            }
        }
    }
}

// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// bad as you would expect
// Rust will not automatically try to convert non-Boolean types to a Boolean
// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }

// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {}", number);
// }

// both arms must have values of the same type
// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is: {}", result);
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {}", element);
//     }
// }

// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }