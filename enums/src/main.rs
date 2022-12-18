enum IpAddrKind {
   V4,
   V6,
}

struct _IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {

}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}


struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = _IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = _IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);

    let loopback = IpAddr2::V6(String::from("::1"));
}
