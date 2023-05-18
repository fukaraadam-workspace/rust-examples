mod copy_clone;
mod function_ownership;
mod reference_borrow;
mod return_ownership;

fn main() {
    // Simple ownership example
    copy_clone::copy_clone_main();
    function_ownership::function_main();
    return_ownership::main();

    // Reference and Borrowing
    reference_borrow::main();
}
