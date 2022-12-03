struct Node<'a> {
    value: u32,
    next: Box<&'a Node<'a>>,
}

fn main() {
    let x: Box<u32> = Box::new(10); // use the stack memory
    println!("{}", *x);
}
