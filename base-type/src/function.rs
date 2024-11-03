fn main() {
    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn f(x: i32) -> i32 {
    x + 1
}
