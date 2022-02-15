// https://docs.rs/nannou/latest/nannou/prelude/fn.random.html

use nannou::prelude::*;

fn main() {
    println!("{:?}", random::<u8>());
    println!("{:?}", random::<i32>());
    println!("{:?}", random::<f64>());
    println!("{:?}", random::<bool>());
}
