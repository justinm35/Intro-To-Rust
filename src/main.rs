#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting: &str = "Nice to meet you";
//     io::stdin().read_line(&mut name)
//         .expect("Didnt Receive Input");
//     println!("Hello {}! {}", name.trim_end(), greeting);
// }

//Strings double quores, char single quores
// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI:f32 = 3.141592;
//     let age = "47";
//     let mut age: u32 = age.trim().parse()
//         .expect("Age wasn't assigned a number");
//     age = age + 1;
//     println!("I'm {} and I want {}", age, ONE_MIL)
// }

//Unsigned integer: u8, u16, u32, u64, u12, usize (Only positve int)
//Signed integer: i8, i16, i32, i64, i128, isize (Can be negative or positive)

// fn main() {
//     println!("Max u32: {}", u32::MAX);
//     println!("Max u64: {}", u64::MAX);
//     println!("Max usize: {}", usize::MAX);
//     println!("Max u128: {}", u128::MAX);

//     println!("Max f32: {}", f32::MAX);
//     println!("Max f64: {}", f64::MAX);

//     let _is_true: bool = true;//Start with underscore to avoid errors on unused vars
//     let my_grade: char = 'A';
// }


// fn main() {
//     let num_1: f32 = 1.111111111111111; //6 digits of precision
//     println!("f32: {}", num_1 + 0.111111111111111);

//     let num_2: f64 = 1.111111111111111;//14 digits of precision
//     println!("f64: {}", num_2 + 0.111111111111111);
// }

// fn main() {
//     let mut num_3: u32 = 5;
//     let num_4: u32 = 4;
//     println!("5 + 4 = {}", num_3 + num_4);
//     println!("5 - 4 = {}", num_3 - num_4);
//     println!("5 * 4 = {}", num_3 * num_4);
//     println!("5 / 4 = {}", num_3 / num_4);
//     println!("5 % 4 = {}", num_3 % num_4);

//     num_3 += 1;
// }

// fn main() {
//     let random_num = rand::thread_rng().gen_range(1..101);
//     println!("Random: {}", random_num);    
// }

// fn main() {
//     let age = 8;
//     if (age >= 1) && (age <= 18) {
//         println!("Important Birthday")
//     }else if (age == 21) || (age == 50) {
//         println!("Important Birthday")
//     }else if age >= 65 {
//         println!("Important Birthday")
//     }else {
//         print!("Not an important Birthday")
//     }
// }

// fn main() {
//     let mut my_age = 47;
//     let can_vote = if my_age >= 18{
//         true
//     }else{
//         false
//     };
//     println!("Can Vote: {}", can_vote)   
// }


// fn main() {
//     let age2: i32 = 8;
//     match age2 {
//         1..=18 => println!("Important Birthday"),
//         21 | 50 => println!("Important Birthday"),
//         64..=i32::MAX => println!("Important Birthday"),
//         _ =>  println!("Not an important Birthday"), //Default Equivelant
//     };
// }

// fn main() {
//     let my_age: i32 = 18;
//     let voting_age: i32 = 18;
//     match my_age.cmp(&voting_age){
//         Ordering::Less  => println!("Can't Vote"),
//         Ordering::Greater => println!("Can Vote"),
//         Ordering::Equal => println!("You gained the right to vote"),
//     };
    
// }

// fn main() {
//     let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
//     println!("1st: {}", arr_1[0]);
//     println!("length: {}", arr_1.len());
// }

// fn main() { //REG LOOP
//     let arr_2 = [1,2,3,4,5,6,7,8,9];
//     let mut loop_idx = 0;
//     loop {
//         if arr_2[loop_idx] % 2 == 0 {
//             loop_idx +=1;
//             continue;
//         }
//         if arr_2[loop_idx] == 9 {
//             break;
//         }
//         println!("Val: {}", arr_2[loop_idx]);
//         loop_idx +=1;
//     }
// }

// fn main() { //WHILE LOOPS
//     let arr_2 = [1,2,3,4,5,6,7,8,9];
//     let mut loop_idx = 0;
//     while loop_idx < arr_2.len(){
//         println!("Arr: {}", arr_2[loop_idx]);
//         loop_idx += 1;
//     }
// }

// fn main() { //FOR LOOPS
//     let arr_2: [i32; 9] = [1,2,3,4,5,6,7,8,9];
//     let mut loop_idx: i32 = 0;
//     for val  in arr_2.iter() {
//         println!("Val : {}", val);
//     }
// }


// fn main() {
//     let my_tuple: (u8, String, f64) = (47, "Justin".to_string(), 50_000.00);
//     println!("Name : {}", my_tuple.1);
//     let(v1, v2, v3) = my_tuple;
//     println!("Age : {}", v1)
// }

fn main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}", word)
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
}
