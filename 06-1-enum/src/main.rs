#[derive(Debug)]

enum IpAddrKind {
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let home2 = IpAddr::V4(127, 0, 0, 1);
    let loopback2 = IpAddr::V6(String::from("::1"));

    // options
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap(); // unwrap() returns the value inside the Some variant
}

fn route(ip_type: IpAddrKind) {
    println!("ip_type is {:?}", ip_type);
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
    // let m = Message::Write(String::from("hello"));
    // m.call();
}