#[derive(Debug)]

enum Ipaddress {
    V4,
    V6,
}

struct Ip_address {
    ip_type: Ipaddress,
    ip_str: String,
}

enum New_ipaddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let my_ip = Ip_address {ip_type: Ipaddress::V4, ip_str: String::from("128.0.0.1")};
    println!("The my ip address is {}", my_ip.ip_str);
    
    let my_new_ip_v4 = New_ipaddress::V4(128, 0, 0, 1);
    let my_new_ip_v6 = New_ipaddress::V6(String::from("::1"));

    println!("The my ip_v4 address is {:?}", my_new_ip_v4);
    println!("The my ip_v6 address is {:?}", my_new_ip_v6);
}
