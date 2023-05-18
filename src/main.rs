mod copy_clone;
mod function_ownership;
mod return_ownership;

fn main() {
    copy_clone::copy_clone_main();
    function_ownership::function_main();
    return_ownership::main();
}
