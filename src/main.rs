//use std::*;
//use rand::Rng;
//use std::cmp::Ordering;

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}