pub fn copy_clone_main() {
    let mut x = 5;
    let y = x; // y is a copy of x
    println!("x = {x}, y = {y}");
    x = 6;
    println!("x = {x}, y = {y}");

    // Clone
    let mut s1 = String::from("hello");
    let s2 = s1.clone(); // s2 is a clone of s1
    println!("s1 = {s1}, s2 = {s2}");
    s1.push_str(", world!");
    println!("s1 = {s1}, s2 = {s2}");

    // Copy
    let s3 = String::from("hello");
    let s4 = s3; // s4 is a copy of s3
                 // println!("s3 = {s3}");   // error[E0382]: borrow of moved value: `s1`
    println!("s4 = {s4}");
}
