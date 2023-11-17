enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Option enum
// enum Option<T> {
//     None,
//     Some(T),
// }

pub fn main() {
    // Enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Enum with struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home address: {}", home.address);

    // Enum with data
    enum IpAddrV2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrV2::V4(127, 0, 0, 1);

    let loopback = IpAddrV2::V6(String::from("::1"));

    // Enum with data and methods
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enum
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}
