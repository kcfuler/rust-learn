fn main() {
    let mut v = vec![1, 2, 3];
    v.push(2);

    // 访问方式
    let num = &v[100]; // panic
    match v.get(100) {
        // 返回Option
        Some(num) => println!("The 100th element is {}", num),
        None => println!("There is no 100th element."),
    }

    // 所有权机制
    let mut v1 = vec![1, 2, 3];
    let ref_to_v1 = &v1[0];
    // 为什么 push 会报错？因为 vector 存在扩容的问题，扩容后原来的引用就失效了
    // v1.push(4); // error

    // 迭代器相关语法
    let v2 = vec![1, 2, 3];
    for i in &v2 {
        println!("{}", i);
        // 解引用语法，可以可以直接修改 vec 中的元素
        println!("{}", *i);
    }

    // 使用枚举来存储不同类型的值
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
