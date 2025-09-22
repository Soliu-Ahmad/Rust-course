// An Enum is a type that can be any one of several variants. Each variant can optionally have data associated with it.
// Enums are defined using the enum keyword, followed by the enum name and a block containing the variants of the enum.
// Enums are useful for representing a value that can be one of several different types, making code more expressive and easier to understand.
// Example of defining and using an enum:

// An Enum is a type that can be any one of several variants. Each variant can optionally have data associated with it.



fn main() {

    enum IpAddrKind {
    V4,
    V6,
}

fn route(_ip_kind: IpAddrKind) {}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
    let _four: IpAddrKind = IpAddrKind::V4;
    let _six: IpAddrKind = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Using Enums
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    // Alternative with struct:
    // struct IpAddr{
    //     kind: IpAddrKind,
    //     address: String,
    // }
    // let home = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
}
    // struct IpAddr{
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home: IpAddr = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    //   let loopback: IpAddr = IpAddr{
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
