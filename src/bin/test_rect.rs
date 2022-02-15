// https://docs.rs/nannou/latest/nannou/prelude/struct.Rect.html

use nannou::prelude::*;
use geom::Range;
use geom::Rect;

fn main() {
    let value = Rect { x: Range::new(-100.0, 100.0), y: Range::new(-200.0, 200.0) };
    println!("{:?}", value);
    println!("{:?}", value.x);
    println!("{:?}", value.y);
    println!("{:?}", value.w());
    println!("{:?}", value.h());
    println!("{:?}", value.w_h());
    println!("{:?}", value.len());
    // println!("{:?}", value.wh());

    let value = Rect::from_w_h(100.0, 200.0);
    println!("{:?}", value);
    println!("{:?}", value.pad(10.0));
    println!("{:?}", value.w_h());
    // println!("{:?}", value.wh());

    let value: Rect = Rect::from_w_h(100.0, 200.0);
    println!("{:?}", value);
}
