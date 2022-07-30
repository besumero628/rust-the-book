enum IpAddKind {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddKind::V4(String::from("127.0.0.1"));
    let six = IpAddKind::V6(String::from("::1"));
}
