// this function could receive any number type
/*
fn sum<T>(a:T, b:T) -> T {
    let x:T = a + b;
    x
}

fn main() {
    let y = sum(12, 20);
    println!("{}", y);
}
*/



use std::ops::Add;

fn sum<T>(a: T, b: T) -> T 
where
    T: Add<Output = T>,
{
    a + b
}

fn main() {
    let y = sum(12, 20);
    println!("{}", y);
}

