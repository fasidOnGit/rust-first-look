use std::f64::consts::PI;
use std::fmt::Debug;

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64
}
#[derive(Debug)]
struct Square {
    side: f64
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.side * self.side;
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

// Type 1 of specifying trait as params
fn print_info(shape: impl Shape + Debug) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn print_info_2<T: Shape + Debug>(shape: T) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}


fn print_info_3<T>(shape: T) where T: Shape + Debug {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

pub fn trait_as_param_main() {
    let c = Circle { radius: 2.0 };
    print_info(c);
}
