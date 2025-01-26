fn main() {
    enum IpAddrKind {
        v4(String),
        v6(String),
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4)


    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let apartment = IpAddr::V4(String::from("127.0.0.1"));
    let backloop = IpAddr::V6(String::from("::1"));

    
}
