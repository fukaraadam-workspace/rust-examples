pub fn main() {
    // Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    let mut s = String::from("hello");
    change(&mut s);

    // At any given time, you can have either one mutable reference or any number of immutable references
    // References must always be valid
    multiple_borrow();
    mutable_immutable_borrow();
    last_used_reference();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multiple_borrow() {
    // Multiple mutable references is not allowed
    let mut s = String::from("hello there");
    let r1 = &mut s;
    let r2 = &mut s;

    // println!("{}", r1); // error[E0499]: cannot borrow `s` as mutable more than once at a time
    println!("{}", r2);
}

fn mutable_immutable_borrow() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);
}

fn last_used_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
