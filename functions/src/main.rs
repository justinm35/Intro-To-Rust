// fn main() {
//     let heap_data = Box::new(42);
//     let new_heap_data = multiply_by_five(heap_data);

//     println!("The heap variable is : {}", new_heap_data);
// }

// fn multiply_by_five(data : Box<i32>) -> Box<i32> {

//     let new_var = Box::new(*data * 5);
//     new_var
// }


fn main() {
    let heap_data = 42;
    let new_heap_data = multiply_by_five(heap_data);

    println!("The heap variable is : {}", new_heap_data);
}

fn multiply_by_five(data : i32) -> i32 {

    let new_var = data * 5;
    new_var
}

