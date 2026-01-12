// Primite data types: int, float, bool, char

/* Inters 
   Rust has signed and unsigned just like C
   Integers can be different sizes.
   signed: i8, i16, i32, i64, i128
   unsigned: u8, u16, u32, u64, u128
*/

fn main(){
  let x: i32 = 42;
  let y:u64 = 8000;
  // the 32 states the size and range of the numbers
  // Remember this stuff from digital logic

  println!("Signed Integer: {}", x);
  println!("Unsigned Integer: {}", y);
  // {} is the placeholder for variables (Does it work for other than ints? check later)

  // Floats
  // f32, f64

  let pi:f64 = 3.14;
  println!("Floating point data type: {}", pi);
}