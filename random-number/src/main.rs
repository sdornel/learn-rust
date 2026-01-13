use rand::prelude::*;

// this prints 100 numbers from 1 to 100
// fn main() {
//     let mut rng = rand::rng();
//     // println!("char: {}", rng.gen::<u8>());
//     // println!("char: '{}'", rng.sample(rand::distr::Alphanumeric));

//     // 1 to 100
//     let mut nums: Vec<i32> = (0..100).collect();
//     // println!("{:?}", nums);

//     // shuffle the numbers
//     nums.shuffle(&mut rng);

//     // let _ = nums.choose(&mut rng);
//     println!("{:?}", nums[0]);
// }

fn main() {
    let y: i8 = rand::random_range(0..100);
    println!("random number: {}", y);
}