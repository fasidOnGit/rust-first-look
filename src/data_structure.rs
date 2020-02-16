use std::mem::size_of_val;
use std::collections::HashMap;

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}
pub fn structures() {
    let p = Point {x: 3.0, y: 4.0};
    println!("point p is at ({}, {})", p.x, p.y);
    let p2 = Point {x: 5.0, y: 10.0};
    let myline = Line {start: p, end: p2};
}

// Enums
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tuple
    Hsl{hue: u8, saturation: u8, lightness: u8}
}

pub fn enums() {
//    let c: Color = Color::RgbColor(120, 128, 128);
    let c:Color =  Color::Hsl {hue: 128, saturation: 50, lightness: 72};
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0) => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({}, {},{})", r,g,b),
        Color::Hsl{hue: _, saturation: _, lightness: 72} => println!("lightness 72"),
        _ => ()
    }
    unions();
}

//Unions
// 32 bits
union IntOrFloat {
    i: i32,
    f: f32
}

pub fn unions() {
    let mut iof = IntOrFloat {i: 123};
    iof.i = 234;
    let value = unsafe {iof.i};
    println!("value = {}", value);
    process_value(IntOrFloat{i: 42});
    process_value(IntOrFloat{f: 52.0});
    process_value(IntOrFloat{i: 5}); // here the 2nd match gets executed and treated as if 5 is a floating point number
    // this is what we would call it in C/C++ as type interpret cast.
    // we will have a crazy result. as an output.!
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            },
            IntOrFloat { f } => {
                println!("value = {}", f);
            }
        }
    }

    options();
}

//Option<T>
pub fn options() {
    let x = 3.0;
    let y = 2.0;
    let result = if y != 0.0 {Some(x/y)} else {None};

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by zero!")
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }

//    while let Some(z) = result {
//        println!("Worked!");
//    }
    array();
}


//Arrays!!

fn array() {
    println!("Array!!");
    let mut a: [i32;5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0]  = 321;
    println!("a[0] = {}", a[0]);

    println!("Fetches the entire array, {:?}", a);

    if a == [321,2,3,4,5] {
        println!("match!");
    }

    // fill the array
    let b = [1; 10];

    for i in 0..b.len() {
        println!("{}", b[i]);
    }
    println!("b took up {} bytes", size_of_val(&b));

    let matrix:[[f32;3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == j {
                println!("matric[{}][{}] = {}", i, j, matrix[i][j]);
            }
        }
    }
    vectors();
}


//Vectors!!!

fn vectors() {
    println!("Vectors - Dynamic arrays.");
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    a.push(44);
    println!("a = {:?}", a);
    // usize and isize.  indices can't be isize. since it must be unsigned.!
    let idx: usize = 0;
    println!("a[0] = {}", a[idx]);
    // get() returns Options!
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("No such element!")
    }

    // iterating vector
    for x in &a {
        println!("{}", x);
    }

    let last_elem = a.pop(); //Option!
    println!("last elemen is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() { // One all the elements are popped out. then it would return None. and the condition will fail\1
        println!("{}", x);
    }
    slices();
}


fn slices() {
    let mut data = [1,12,3,4,5];
    use_slice(&mut data[1..4]);
//    use_slice(&mut data);
    println!("{:?}", data);
    strings();
}

fn use_slice(slice: &mut[i32]) {
    println!("first elem of {}, len = {}", slice[0], slice.len());
    slice[0] = 4521;
}

fn strings() {
    let s = "hello there!"; //&str = string slice.
    for c in s.chars() {
        println!("{}", c);
    }

    if let Some (first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    //heap
    //String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str <> String
    let u:&str = &letters;

    //concatentaion
    // String + str
    let z =  letters.to_owned() + "abc";
    //String + String
    let zz = letters.to_owned() + &letters; //De-ref conversion to cast String to &str

    let mut abc = "hello".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
    tuples();
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}


fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //destructuring tuple;
    let (a,b) = sp;
    let sp2 = sum_and_product(4,7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

//    even more destructring
    let ((c,d), (e,f)) = combined;
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);
    let meaning = (42,);
    println!("{:?}", meaning);

    hash_maps();
}


fn hash_maps() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);
    println!("a triangle has {} sides", shapes["triangle".into()]);

    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);
    println!("Before {:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("After {:?}", shapes);
}
