#[derive(Debug)]

// Enum is basically a type to define your own variants
enum IpAddress{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ip = IpAddress::V4(123, 040, 155, 120);
    let ip2 = IpAddress::V6(String::from("x2345"));

    println!("{:?}", ip);
    println!("{:?}", ip2);
}