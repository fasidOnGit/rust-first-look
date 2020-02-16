pub fn print_value(x: i32) {
    let mut z = 1;
    increase(&mut z);
    println!("z={}", z);
    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);

    let a = Point { x: 0f64, y: 0f64 };
    let b = Point { x: 1.2, y: 3.4 };
    let myline = Line {start: a, end: b};
    println!("Length of my line is {}", myline.len());
    closures();
}

struct Point {
    x: f64,
    y: f64
}
struct Line {
    start: Point,
    end: Point
}
impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y:i32) -> i32 {
    x*y
}


fn closures () {
    let sh = product;
    sh(34,34);
    let plus_one = |x:i32| -> i32 {
        x + 1
    };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));
    let mut two = 2;
    // The reason for the a new scope below is that, a
    // You cannot borrow a variable second time.
    // so, as you can see, we are infact borrowing the variable two inside ths scope.
    // whilst trying to borrow it again i borrow_two variable below the scope.
    // the presence of this scope will actually let two be released after it has exited the scope.
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }
    let borrow_two = &mut two;

    //T: by value
    //T&  -> be ref
    // &mut &  -> mutable ref
    let plus_three = |x:&mut i32| *x +=3;
    let mut f = 12;
    plus_three(&mut f); // assigned by ref
    let mut aa = f;  // assigned by value
    aa += 1;
    println!("f = {}, aa = {}", f, aa);
    higher_order_function();
}

//functions that take functions
// f(g) {let x = g(); }
// functions that return functions.
// generators
// f () -> g
fn higher_order_function () {
    // sum of all even squares < 500
    let limit = 500;
    let mut sum = 0;

//    let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i*i;
        if above_limit(isq) {
            break;
        }
        else if is_even(isq) {
            sum += isq;
        }
    }
    println!("loop sum = {}", sum);

    // Functions that takes functions!;
    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum2);
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}
