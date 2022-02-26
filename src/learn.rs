/**
 * Integers: u8, i8, u16, i16, ... u128, i128
 * Floats: f32, f64 
 * Boolean: bool
 * Characters: char
 * Tuples
 * Arrays
 */

use std::io;

pub fn run() {
  vectors_array();
  let mut count = 0; 
  loop {
    count += 1; 
    println!("Number: {}", count);

    if count == 20 {
      break; 
    }
  }

  // While Loop  (Fix)
  while count <= 100 {
    if count % 15 == 0 {
      println!("Fizzbuzz: {}", count);
    } else if count % 3 == 0{
      println!("Fizz: {}", count);
    } else if count % 5 == 0 {
      println!("Buzz: {}", count);
    } else {
      println!("{}", count);
    }

    count += 1; 
  }
}

fn vectors_array() {
  let mut vectors: Vec<i32> = vec![1, 2];
  let numbers: [i32; 2] = [2, 3]; 
  for n in 0..numbers.len() {
    println!("{}", numbers[n]);
    vectors.push(numbers[n]);
  }
  println!("{:?}", vectors);
}

fn input_name() {
  let mut user_name = String::new(); 
  io::stdin().read_line(&mut user_name).expect("Failed to get user name!");
}

fn learn_string() {
  let mut name = String::from("Stephen");
  println!("Name of software engineer writing this program is {} \nand length of name is {}", name, name.len());
  name.push_str(" Pham");
  println!("New name {}", name);
  println!("Capacity of {} is {}", name, name.capacity());
  for word in name.split_whitespace() {
    println!("Word: {}", word);
  }

  let mut result = String::new(); 
  result.push('A');
  assert_eq!(1, result.len());
  println!("Successfully with result: {}", result);
}

fn learn_tuples() {
  let person: (&str, &str, i64) = ("Stephen", "Viet Nam", 20);
  println!("{} is from {} and is {} year old", person.0, person.1, person.2);
}