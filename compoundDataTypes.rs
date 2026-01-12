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
}

