#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // greet_user();
    // var_demo();
    // data_types_demo();
    // ternary_demo();
    // match_demo();
    array_demo();
}

pub fn greet_user() {
    let mut name = String::new();
    let greeting = "Nice to meet you";

    println!("What is your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");
    
    println!("{} {}!", greeting, name.trim_end());
}

pub fn var_demo() {
    // unsighed 32-bit int
    const ONE_MIL: u32 = 1_000_000;
    // float 32-bit
    const PI: f32 = 3.141592;
    // float 32-bit
    let age = "29";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number.");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

pub fn data_types_demo() {
    println!("Max u32: {}", u32::MAX);
    println!("Min u32: {}", u32::MIN);
    println!("Max i32: {}", i32::MAX);
    println!("Min i32: {}", i32::MIN);
    println!("Max u64: {}", u64::MAX);
    println!("MIN u64: {}", u64::MIN);
    println!("Max i64: {}", i64::MAX);
    println!("Min i64: {}", i64::MIN);
    // Output:
    // Max u32: 4294967295
    // Min u32: 0
    // Max i32: 2147483647
    // Min i32: -2147483648
    // Max u64: 18446744073709551615
    // MIN u64: 0
    // Max i64: 9223372036854775807
    // Min i64: -9223372036854775808
    let is_true = true;
    let my_grade = 'A';
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);
    // Output:
    // f32: 1.2222223
    // f64: 1.2222222222222219
    let random_num: u64 = rand::thread_rng().gen_range(1..101);
    println!("random: {}", random_num);
    
    let age = "30";
    let age: u32 = age.trim().parse().expect("Unable to change age to number");
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday: {}", age)
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday: {}", age)
    } else if age >= 65 {
        println!("Important Birthday: {}", age)
    } else {
        println!("NOT an Important Birthday: {} looserlooserlooser~", age)
    }
}

pub fn ternary_demo() {
    let my_age: i32 = 30;
    let can_vote = if my_age >= 18 { true } else { false };
    // This is so fucking sick.
    println!(
        "{} years old ability to vote is {}",
        my_age, 
        if can_vote { "valid" } else { "invalid" }
    );
}

pub fn match_demo() {
    let my_age: i32 = 18;
    let voting_age:i32 = 18;
    let age2: i32 = 8;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("{} is not old enough!", my_age),
        Ordering::Equal => println!("By the skin of your teeth. {}", my_age),
        Ordering::Greater => println!("{} is old enough to vote", my_age)
    }
    match age2 {
        1..=18 =>  println!("Important Birthday: {}", age2),
        21 | 50 => println!("Important Birthday: {}", age2),
        65..=i32::MAX => println!("Important Birthday: {}", age2),
        _ =>  println!("NOT an Important Birthday: {} looserlooserlooser~", age2)
    }
}

pub fn array_demo() {
    let arr_1 = [1,2,3,4];
    println!("1st: {}", arr_1[0]);
    // Get the length
    println!("length: {}", arr_1.len());
    let mut arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            // restarts the loop, the rest of the code in the loop will not execute.
            continue;
        }
        if arr_2[loop_idx] == 9 {
            // exits the loop && stops its execution.
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1
    }
}