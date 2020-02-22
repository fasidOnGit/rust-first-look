use std::f64::consts::PI;

trait Printable {
  fn format(&self) -> String;
}
impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}
pub fn dispatch_invocation() {
    let a = 123;
    let b = "hellow".to_string();
    println!("{}", a.format());
    println!("{}", b.format());
    print_it(a);
    print_it(b);
    dynamic_dispatch();
    //This is Static Dispatch.
    // How is it related to dispatch..
    // Well if you take a look at print_it.
    // z can be of anything that implments Printable.
    // So z can be i32, String and whatnot
    // So essentially at compile time, rust will figure out
    // to use a concrete implementation of print_it.
    // so for a, it's pretty much like, compiler will use print_it(z: i32)
    // and for b, it's print_it(z: String)
}


//Dynamic Dispatch.
fn print_it_too(z: &dyn Printable) {
    println!("{}", z.format());
}

// Here is what happens,
// when calling this function, the references that are feeded into the function loses thier types at compilations
// so print_it_too(3), Would lose the type of i32 when getting into the function. So, calling  impl Printable for i32 is not figured out at compile time
// but at the run time..
// so basically the types of the variables,are inferred at the runtime, and hence it's an expensive operation..


// Why Dynamic Dispatch.

struct Circle { radius: f64 }
struct Square { side: f64 }

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

fn dynamic_dispatch() {
    let shapes: [&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0},
    ];
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area());
    }
}
