fn main() {
  struct Person {
    age: u32,
    exercise: bool,
    name: String,
    notes: String,
  }

  let mut new_person = Person {
    age: 12,
    exercise: false,
    name: String::from("Johnathan"),
    notes: String::from("Nothingness")
  };
  
  new_person.exercise = true;

  println!("{}'s exercise status is: {}", new_person.name, new_person.exercise);

  //struct update syntax
  let new_person_2 = Person {
    name: String::from("Richard"),
    ..new_person
  };

  println!("{}'s exercise status is: {}", new_person_2.name, new_person_2.exercise);
  println!("{}'s exercise status is: {}, age are: {}", new_person.name, new_person.exercise ,new_person.age, );
  //have access to the u32 value sinc eit implements the copy trait

  //-----------------------------------------------------------------------------------------


  struct Color(i32, i32, i32); // This is an example of a tuple struct
  struct Point(i32, i32, i32); // Tuple structs are advantageous because of their ability to be named.

  fn add_color(color: &Color) {
    println!("{},{},{}", color.0, color.1, color.2);
  };

  let red = Color(50, 225, 255);
  let cross_roads = Point(5, 5, 5);

  add_color(&red);
  // add_color(&cross_roads); // Throws error since it is the wrong type of tuple despite being the same set of types
}



