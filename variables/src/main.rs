
// fn main() {
//     let s = "Hello, world!";
//     let v: Vec<String> = s.chars().map(|c| c.to_string()).collect();
//     for str in v {
//         println!("{}", str);
//     }
// }



// //SLICESSS

// fn main() {
//     let first_string = String::from("Hi Peter.");

//     let name_slice =  &first_string[3..8];

//     println!("{}",name_slice);

//     println!("Memory adress is {:p}", &first_string)

// }
// fn main() {
//     fn first_word(s : &String) -> usize {
//         let bytes = s.as_bytes(); //converts a string to an array of bytes.
    
//         for (i, &item) in bytes.iter().enumerate() { //first item of tuple is an index and the second is a reference ot the element
//             if item == b' ' {
//                 return i;
//             }
//         }
//         s.len()
//     }
    
//     let my_string = String::from("Hello World");

//     first_word(&my_string);
    
//     my_string.clear()//will not throw error but we will no longer be able to reference this anymore
// }


//Slices solve this
fn main() {

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]
            }
        }
        &s[..]
    }

    let my_string = String::from("Hello World");

    let word = first_word(&my_string);
    println!("The word is : {}", word);


    let my_string_lit = "Hey Justin!";

    let word2 = first_word(my_string_lit);
    println!("The word2 is : {}", word2)

}