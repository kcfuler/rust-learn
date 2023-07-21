fn greet_world() {
    let southern_germany = "lalala🍺";
    let chinese = "世界 你好！";
    let english = "hello world!";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn check_float() {
    assert!(0.1 + 0.2 == 0.3);
}

fn main() {
    // greet_world();
    // let mut x = 5;
    // x = 6;
    // println!("{}", x);
    check_float();
}
