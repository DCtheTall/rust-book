fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let _home = IpAddrStruct{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddrStruct{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home = IpAddrTypes::V4(127, 0, 0, 1);
    let _loopback = IpAddrTypes::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // enum Option<T> {None, Some(T)}
    let _some_number = Some(5);
    let _some_string = Some(String::from("hello"));
    let _absent_number: Option<i32> = None;

    // Compiler error, cannot operatore on the value of Optional that way
    // let x: i8 = 1;
    // let y: Option<i8> = Some(1);
    // println!("sum is {}", x + y);
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(_ip_kind: IpAddrKind) {
    // ...
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

// Enum variants can be typed
enum IpAddrTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enum variants can also be structs
struct Ipv4Addr;
struct Ipv6Addr;
enum IpAddrStructs {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// This enum...
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Is equivalent to...
struct QuitMessage;
struct MoveMessage{x: i32, y: i32}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// You can also define methods for enums
impl Message {
    fn call(&self) {
        // ...
    }
}
