// Compound data types
// arrays, tuples, slices, strings and slice string

// Arrays
// like in C, and array needs have homogenoeous types and fixed size

fn main() {
  let numbers:[i32; 5] = [1, 2, 3, 4, 5];
  // Array format: var [datatype; size]

  // println!("Array: {}", numbers);
  // In rust, there are debuggable and display format, will be taught later

  println!("Array: {:?}", numbers);
  // {:?} is one of many ways to express it

  let fruits:[&str; 3] = ["Apple", "Banana", "Orange"];

  println!("Fruits: {:?}", fruits);
  println!("Fruits: {}", fruits[0]);
  println!("Fruits: {}", fruits[1]);
  println!("Fruits: {}", fruits[2]);

  // Tuples
  // Tuples are heterogenous elements of fixed size
  let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
  println!("Human Tuple: {:?}", human);
  // String literals are not strings, need .to_string method

  let mixedTuple = ("James", 23, true, [1, 2, 3, 4, 5, 6]);
  println!("Mixed Tuple: {:?}", mixedTuple);

  // Slices = [1, 2, 3, 4, 5]
  let numberSlices:&[i32] = &[1, 2, 3, 4, 5];
  println!("Number Slice: {:?}", numberSlices);

  let animal_slices:&[&str] = &["Lion", "Elephant", "Crocodile"];
  println!("Animal slices: {:?}", animal_slices);

  let book_slices:&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Lord of the Rings".to_string()];
  println!("Book slices: {:?}", book_slices);

  /* String vs String Slice
     String is growable/mutable. They are called owned
     Rust is great for speed and low-level trade off 
     Any data type in rust is immutable*/

     let mut stone_cold:String = String::from("Hell "); // This is stored in the heap
     println!("Stone Cold Says: {}", stone_cold);
     stone_cold.push_str("Yeah!");
     println!("Stone Cold Says: {}", stone_cold);

     // &str is a string slice/ reference

     let string:String = String::from("Hello, World!");
     let slice:&str = &string[0..5];

     println!("String: {}", string);
     println!("Slice Value: {}", slice);
     println!("Slice Value: {:?}", slice);


}