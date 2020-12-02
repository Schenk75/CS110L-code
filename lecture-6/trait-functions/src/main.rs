use std::fmt;

fn identity_fn<T>(x: T) -> T {x}

fn print_excited<T: fmt::Display>(x: T) {
    println!("{}!!!!!!!!!!!!!!!! :DDDD", x);
}

fn print_min<T: fmt::Display + PartialOrd>(x: T, y: T) {
    if x < y {
        println!("The minimum is {}", x);
    } else {
        println!("The minimum is {}", y)
    }
}

fn main() {
    println!("{:?}", identity_fn(Some(110)));
    print_excited(110);
    print_min(1, 10)
}