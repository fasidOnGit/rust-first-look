pub fn pattern_match() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let mut point = (3,4);
    match point {
        (0,0) => println!("Origin"),
        (0,y) => println!("x axis,y = {}", y),
        (ref mut x,0) => println!("y axis,y = {}", x),
        (x,y) => println!("{}, {}", x, y)
    }
}

fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9..=11 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}
