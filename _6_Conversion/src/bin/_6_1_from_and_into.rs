use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let n1 = Number::from(10);   // uses From<i32> for Number
    let x: i32 = 20;
    let n2: Number = x.into();   // uses Into<Number> for i32, auto from From

    println!("{:?}", n1);
    println!("{:?}", n2);
}
