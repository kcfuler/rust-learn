mod function;
mod stl;

fn main() {
    let mut sum = 0;
    for i in 0..10 {
        sum += i;
    }
    println!("sum: {}", sum);
}
