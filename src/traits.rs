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
}
