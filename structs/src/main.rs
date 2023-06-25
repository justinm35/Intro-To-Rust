// fn main() {
//   struct Person {
//     age: u32,
//     exercise: bool,
//     name: String,
//     notes: String,
//   }

//   let mut new_person = Person {
//     age: 12,
//     exercise: false,
//     name: String::from("Johnathan"),
//     notes: String::from("Nothingness")
//   };
  
//   new_person.exercise = true;

//   println!("{}'s exercise status is: {}", new_person.name, new_person.exercise);

//   //struct update syntax
//   let new_person_2 = Person {
//     name: String::from("Richard"),
//     ..new_person
//   };

//   println!("{}'s exercise status is: {}", new_person_2.name, new_person_2.exercise);
//   println!("{}'s exercise status is: {}, age are: {}", new_person.name, new_person.exercise ,new_person.age, );
//   //have access to the u32 value sinc eit implements the copy trait

//   //-----------------------------------------------------------------------------------------


//   struct Color(i32, i32, i32); // This is an example of a tuple struct
//   struct Point(i32, i32, i32); // Tuple structs are advantageous because of their ability to be named.

//   fn add_color(color: &Color) {
//     println!("{},{},{}", color.0, color.1, color.2);
//   };

//   let red = Color(50, 225, 255);
//   let cross_roads = Point(5, 5, 5);

//   add_color(&red);
//   // add_color(&cross_roads); // Throws error since it is the wrong type of tuple despite being the same set of types
// }


fn main() {
  // //-----------Implementation with normal variables ---------------
  // let width = 30;
  // let height = 50;

  // println!("The area of the rectangle is {} square pixels", area(width, height));

  // fn area(width: u32, height: u32) -> u32{
  //   width * height
  // }


// //-----------Implementation with tuples ---------------
//   let rect1 = (30, 50); //This is a tuple

//   println!(
//     "The area of the rectangle is {} square pixels", 
//     area(rect1)
//   );

//   fn area(rect_dim: (u32, u32)) -> u32{
//     rect_dim.0 * rect_dim.1
//   }

  //-----------Implementation with structs ---------------
  struct Rectangle {
    width: u32, 
    height: u32,
  }

  let my_rectangle = Rectangle{
    width: 30,
    height: 50
  };

  println!(
    "The area of the rectangle is {} square pixels", 
    area(&my_rectangle)
  );

  fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
  }

}
