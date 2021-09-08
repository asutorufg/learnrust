#[derive(Debug)]
pub enum IpAddrKind<T> {
    V6(T),
    V4(T),
}

pub fn route(ip_kind: IpAddrKind<String>) {
    println!("{:?}", ip_kind);
}

pub fn test10() {
    let one = IpAddrKind::V4(String::from("127.0.0.1"));
    let two = IpAddrKind::V6(String::from("::ff"));

    route(one);
    route(two);
}

pub fn test11(ip_kind: IpAddrKind<String>) {
    match ip_kind {
        IpAddrKind::V4(_) => {
            println!("V4 -> {:?}", ip_kind);
        }
        IpAddrKind::V6(_) => {
            println!("V6 -> {:?}", ip_kind)
        }
    }
}
