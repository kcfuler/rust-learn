enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 将数据直接附加到枚举的变体中
// 优点：不需要额外使用 struct
// 缺点: 每个变体可以拥有不同的类型以及关联的数据量
enum IpAddr2 {
    V4(String),
    V6(String),
}

// 同样使用impl关键字来定义方法
impl IpAddr2 {
    fn call(&self) {
        println!("call");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 使用IpAddr2
    let home = IpAddr2::V4(String::from(""));

    // 使用Option
    let op = Option::Some(1);
    let unknown: Option<i32> = Some(1);

    let result = unknown.unwrap_or(0) + 1;

    home.call();
}
