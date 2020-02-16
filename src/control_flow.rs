pub fn if_statement() {
    println!("If block!");
    let temp = 45;
    if temp > 30 {
        println!("really hot outside!");
    } else if temp < 10 {
        println!("pretty cold!");
    } else {
        println!("termperature is OK!!");
    }


    let day = if temp > 20 {"sunny"} else {"cloudy"};  // braces are mandatory!
    println!("today is {}", day);

    println!("is it {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"}
    )
}

pub fn while_and_loop() {
    println!("While Loop");
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 { continue; }
        println!("{}", x);
    }

    println!("Infinite Loop");

    let mut y = 1;
    loop { // executes forever...
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 { break; }
    }
    for_loop();
}

pub fn for_loop() {
    println!("For Loop");
    // 1..11 , the .. fetches all the number within the range EXCLUDING 11. whereas 1...11 is INCLUSIVE of 11.
    // alright INCLUSIVENESS can also be acheived by 1..=11
    for x in 1..11 {
        println!("x = {}", x);
    }

    println!("Grab the index.!");
    for (index, y) in (30..41).enumerate() {
        println!("{}: {}", index, y);
    }
    match_statement();
}

pub fn match_statement() {
    println!("Match a.k.a Switch! statement!");
    let country_code = 91;
    let country = match country_code {
        91 => "India",
        44 => "UK",
        7 => "Russia",
        1..=999 => "unknown",
        _ => "invalid" // pretty much the default in switch!
    };
    println!("the country with code{} is {}", country_code, country);
}
