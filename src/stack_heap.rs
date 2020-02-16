#![allow(dead_code)]

use std::mem::size_of_val;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y:0.0}
}
pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes up {} bytes", size_of_val(&p1)); // --> stacked value
    println!("p2 takes up {} bytes", size_of_val(&p2)); // --> Stacked value
    println!("p2 takes up {} bytes", size_of_val(&*p2)); // --> Heaped value
}
