#![allow(unused)]

use core::num;
use rand::Rng;
use std::any::type_name;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;

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
    println!(
        "{} years old ability to vote is {}",
        my_age,
        if can_vote { "valid" } else { "invalid" }
    );
}

pub fn match_demo() {
    let my_age: i32 = 18;
    let voting_age: i32 = 18;
    let age2: i32 = 8;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("{} is not old enough!", my_age),
        Ordering::Equal => println!("By the skin of your teeth. {}", my_age),
        Ordering::Greater => println!("{} is old enough to vote", my_age),
    }
    match age2 {
        1..=18 => println!("Important Birthday: {}", age2),
        21 | 50 => println!("Important Birthday: {}", age2),
        65..=i32::MAX => println!("Important Birthday: {}", age2),
        _ => println!("NOT an Important Birthday: {} looserlooserlooser~", age2),
    }
}

pub fn array_looping_demo() {
    let arr_1 = [1, 2, 3, 4];

    println!("1st: {}", arr_1[0]);
    println!("length: {}", arr_1.len());

    let mut arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;

    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            // restarts the loop, the rest of the code in the loop will not execute.
            continue;
        }
        if arr_2[loop_idx] >= 9 {
            // exits the loop && stops its execution.
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1
    }

    loop_idx = 0;
    print!("Array 2 contains: ");
    while loop_idx < arr_2.len() {
        print!("{} ", arr_2[loop_idx]);
        loop_idx += 1;
    }

    print!("\nArray 2 contains: ");
    for val in arr_2.iter() {
        print!("{} ", val);
    }
}

pub fn tuple_demo() {
    let my_tuple: (u8, String, f64) = (30, "Jerome".to_string(), 68_000.00);
    println!(
        "Name: {}, Age: {}, Something: {}",
        my_tuple.1, my_tuple.0, my_tuple.2
    );
    // A perfect example of pattern matching.
    let (age, name, monies) = my_tuple;
    println!("Name: {}, Age: {}, Something: {}", name, age, monies);
}

pub fn string_demo() {
    let mut char_name = String::new();
    let mut desired_char_name = String::new();
    char_name.push('C');
    char_name.push_str("hara");
    println!("What is your characters name?");
    io::stdin()
        .read_line(&mut desired_char_name)
        .expect("No name entered.");

    desired_char_name = desired_char_name.trim_end().to_string();

    println!(
        "No... your name is not {}, its {}.",
        desired_char_name, char_name
    );
    for word in desired_char_name.split_whitespace() {
        println!("{},", word);
    }

    let replaced: String = desired_char_name.replace("Jerome", "Chara");
    for word in replaced.split_whitespace() {
        println!("{},", word);
    }

    let friend = String::from("YouYouYouYouYou");
    let mut v1: Vec<char> = friend.chars().collect();
    v1.sort();
    v1.dedup();
    for chara in v1 {
        println!("{}", chara);
    }

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let btye = st5.as_bytes();
    let st6 = &st5[0..=5];
    println!("String length: {}", st6.len());

    st5.clear();
}

pub fn casting_demo() {
    let int1_u8: u8 = 8;
    let int2_u8: u8 = 4;
    let int1_u32: u32 = (int1_u8 as u32) + (int2_u8 as u32);
}

pub fn enum_demo() {
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Sunday;
    match today {
        Days::Monday => println!("I hate Mondays ;w;"),
        Days::Tuesday => println!("Donut day?"),
        Days::Wednesday => println!("Hump day"),
        Days::Thursday => println!("void"),
        Days::Friday => println!("I'm so close!"),
        _ => println!("Weekend"),
    }

    println!(
        "Is today the weekend?... {}",
        if today.is_weekend() { "YES" } else { "NUH" }
    );
}

pub fn vectors_demo() {
    // vectors are like arrays, in that they can grow if mutable.
    // they can only store values of the same type.
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    let mut vec3 = vec![5, 6, 7];
    vec2.append(&mut vec3.clone());
    assert_eq!(vec2, vec![1, 2, 3, 4, 5, 6, 7]);
    vec3.append(&mut vec2.clone());
    println!("vec3:");
    for ele in &vec3 {
        println!("{}", ele);
    }
    vec3.sort();
    println!("\nsorted vec3:");
    for ele in &vec3 {
        println!("{}", ele);
    }

    // Add values to the end of a vector
    vec1.push(5);
    vec1.push(55);
    vec1.push(555);
    vec1.push(-5);

    println!("3rd : {}", vec2[2]);

    // Verify value exists
    let second: &i32 = &vec2[1];

    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value in vec2."),
    };

    let mut i = 0;
    for val in &mut vec2 {
        *val *= 2;
        i += 1;
        println!("idx: {}, val: {}", i, val);
    }

    let mut i = 0;
    for val in &mut vec2 {
        i += 1;
        println!("idx: {}, val: {}", i, val);
    }

    // Get number of values in a vector
    println!("Vec Length : {}", &vec2.len());

    let mut i = 0;
    for val in &mut vec2 {
        i += 1;
        println!("idx: {}, val: {}", i, val);
    }

    // Remove and return the last value
    println!("Pop {:?}", &mut vec2.to_vec().pop());
    println!("Vec Length : {}", vec2.len());

    // Remove and return the last value
    println!("Pop {:?}", vec2.clone().to_owned().pop());
    println!("Vec Length : {}", vec2.len());

    // Remove and return the last value
    println!("Pop {:?}", &mut vec2.pop());
    println!("Vec Length : {}", vec2.len());
}

fn functions_demo() {
    let x = 50;
    let (plus, times) = get_two_from_1(x);
    println!("{}, plus 2: {}, times 2: {}", x, plus, times);

    let num_list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = sum_list(&num_list);
    let num_list_squared: Vec<i32> = num_list.iter().map(|x| x * x).collect();
    num_list_squared.iter().for_each(|x| println!("{}", x));
}

fn get_two_from_1(x: i32) -> (i32, i32) {
    return (x + 2, x * 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let sum = list.into_iter().sum();
    println!("sum: {}", sum);
    return sum;
}

fn generics_demo() {
    let (a, b, c) = (5.2, 4.7, 9.9);
    let (x, y, z) = (5, 4, 9);
    let test_one = assert_eq!(c, get_sum_gen(a, b));
    println!("{}", type_of(test_one));
    println!("{}", type_of(&test_one));
    println!(
        "{} + {} = {}, this is {:#?}",
        a,
        b,
        c,
        assert_eq!(c, get_sum_gen(a, b))
    );
    println!(
        "{} + {} = {}, this is {:#?}",
        x,
        y,
        z,
        assert_eq!(z, get_sum_gen(x, y))
    );
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn ownership_demo() {
    let str1 = String::from("World");
    let str2 = str1.clone();
    println!("{}", str1);
    ps(str1.clone());
    let mut str3 = prs(str1);
    cs(&mut str3);
}

fn ps(x: String) {
    println!("test: {}", x);
}

fn prs(x: String) -> String {
    println!("test: {}", x);
    x
}

fn cs(x: &mut String) {
    x.push_str(" is a string");
    println!("message: {}", x);
}

// fn player_starts() -> bool {
//     println!("Who will start (me/you)");
//     loop {
//         let input = io::stdin().read_line(">");
//         match input.expect("Failed to read line").as_ref() {
//             "me" => return true,
//             "you" => return false,
//             _ => println!("Enter me or you"),
//         }
//     }
// }

fn hash_demo() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayen");
    heroes.insert("Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    println!("Length of HashMap: {}", heroes.len());

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("A... {}", x),
            None => println!("eh."),
        }
    }
}

fn struct_demo() {
    struct Status {
        name: String,
        def: String,
        level: u32,
    }
    let mut hark = Status {
        name: String::from("(u.u)"),
        def: String::from("sleepsleepsleepsleepsleep"),
        level: 33,
    };
    hark.def = String::from("..");
    println!("{}", hark.def);
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };
}

fn trait_demo() {
    const PI: f32 = 3.141592;
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle {
        length: f32,
        width: f32,
    }
    struct Circle {
        length: f32,
        width: f32,
    }
    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let cir: Circle = Shape::new(10.0, 10.0);
    println!(
        "rectangle area: [{}], circle area: [{}]",
        rec.area(),
        cir.area()
    );
}


fn err_and_file_demo() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file: {:?}", error)
    };
    write!(output, "Just some\nRandom Words").expect("Failed to write to file.");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");

    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_errors => panic!("Problem opening file: {:?}", error),
        },
    };
}

mod restaurant;
use crate::restaurant::{meal::Meal, order_food};

fn main() {
    // greet_user();
    // var_demo();
    // data_types_demo();
    // ternary_demo();
    // match_demo();
    // array_looping_demo();
    // tuple_demo();
    // string_demo();
    // casting_demo();
    // enum_demo();
    // vectors_demo();
    // functions_demo();
    // generics_demo();
    // ownership_demo();
    // hash_demo();
    // struct_demo();
    // trait_demo();
    // order_food(Meal::Lunch);
    // order_food(Meal::Dinner);
    // order_food(Meal::Breakfast);
    err_and_file_demo();

}
