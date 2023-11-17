const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn type_tuple() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of y is: {y}");
  let five_hundred = tup.0;
  let six_point_four = tup.1;
  let one = tup.2;
}

pub fn type_array() {
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let a = [3; 5]; // [3, 3, 3, 3, 3]
}
pub fn types_main() {
  // Mutability
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");

  // Shadowing
  let y = 5;
  let y = y + 1;

  let spaces = "   ";
  let spaces = spaces.len();

  // Data types
  let x = 2; // i32
  let guess: u32 = "42".parse().expect("Not a number!");

  let x = 2.0; // f64
  let y: f32 = 3.0; // f32

  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1
  let remainder = 43 % 5;
  let t = true;
  let f: bool = false; // with explicit type annotation
  let c = 'z';
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';
}