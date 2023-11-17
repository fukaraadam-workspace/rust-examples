fn main() {
    println!("Hello, world!");
    let my_struct = MyStruct {
        my_variable: String::from("hello"),
    };

    println!("Constant: {}", my_struct.get_collection_id());

    // let collection_id = my_struct.get_collection_id(2);
}

struct MyStruct2<T: MyTrait> {
    my_struct: T::MyType,
}

trait MyTrait {
    type MyType: Copy;
    const MY_CONSTANT: Self::MyType;

    fn get_collection_id(&self) -> Self::MyType;
}

struct MyStruct {
    my_variable: String,
}

impl MyTrait for MyStruct {
    type MyType = i32;
    const MY_CONSTANT: Self::MyType = 24;

    fn get_collection_id(&self) -> Self::MyType {
        Self::MY_CONSTANT
    }
}