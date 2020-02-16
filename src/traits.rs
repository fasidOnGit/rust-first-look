trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) { println!("Default Implementation - {} cannot talk", self.name())}
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self { result += *x;}
        return result;
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name:&'static str) -> Human {
        Human{ name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name:&'static str) -> Cat {
        Cat{ name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}
pub fn traits_fun() {
//    let h = Human{name:"John"};
    let h = Human::create("John");
    h.talk();

//    let c = Cat{name: "Misty"};
    //yet another method of doing the same
    let c: Cat = Animal::create("Misty");
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
    into_main();
    drop_trait();
}


struct Person {
    name: String
}

impl Person {
//    fn new(name: &str) -> Person {
//        Person { name: name.to_string()}
//    }
    fn new<S: Into<String>>(name: S) -> Person {
        Person {name: name.into()}
    }

    fn new_where<S>(name:S) -> Person where S: Into<String> {
        Person { name: name.into()}
    }
}

fn into_main () {
    let john = Person::new("John");
    let name: String = "Jane".to_string();
    let jane = Person::new(name);
}

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature {name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

fn drop_trait() {
    let goblin = Creature:: new("Jeff");
    println!("game proceeds!");
    drop(goblin);
}
