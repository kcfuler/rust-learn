/*
 rust 不支持对单个变量指定可变性，只能对整个结构体指定可变性
*/
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct
struct Color(i32, i32, i32);

// unit-like struct
// 适用于没有字段的情况，类似于空对象模式，可以用它来实现一些工具方法（不需要数据）
struct UnitLikeStruct;

struct Square {
    width: u32,
    height: u32,
}

impl squre {
    fn from(width: u32, height: u32) -> squre {
        squre { width, height }
    }
}

fn main() {
    let active = true;
    // 初始化
    let user1 = User {
        email: String::from(" [email protected]"),
        username: String::from("user1"),
        active, // 简写
        sign_in_count: 1,
    };

    // struct update syntax
    let user2 = User {
        email: String::from(" [email protected]"),
        username: String::from("user2"),
        ..user1 // 这里和JS的对象合并很像，只不过这里是放在大括号的末尾
    };

    println!("User1 email: {} {}", user1.email, user2.username);
}
