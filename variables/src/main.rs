fn main() {
    fn first_word(s : &String) -> usize {
        let bytes = s.as_bytes(); //converts a string to an array of bytes.
    
        for (i, &item) in bytes.iter().enumerate() { //first item of tuple is an index and the second is a reference ot the element
            if item = b'' {
                return i;
            }
        }
        s.len
    }


    let my_string = String::from("Hello World");
    
    first_word(&my_string);

}