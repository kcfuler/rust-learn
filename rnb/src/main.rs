fn main() {
    println!("Hello, world!");
    let m1 = String::from("m1");
    let m2 = String::from("m2");
    greet(&m1, &m2);
    println!("{} and {}", m1, m2); // error[E0382]: borrow of moved value: `m1`

    let mut v = vec![1, 2, 3];
    let num = &mut v[2];
    // let num2 = &*num; // 引用在同一时间内被多次借用后，不可写
    *num += 1;
    // v.push(3);
    println!("num: {}", *num); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
}
fn greet(m1: &String, m2: &String) {
    println!("greet: {} and {}", m1, m2);
}

fn return_a_string() -> &String {
    let s = String::from("hello");
    let ref_s = &s;

    ref_s
}

fn slice() {
    let s = String::from("hello");
    let slice = &s[0..2];
    println!("slice: {}", slice);
    // s.clear(); // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("slice: {}", slice);
}
