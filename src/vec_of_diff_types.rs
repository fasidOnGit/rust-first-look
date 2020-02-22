use crate::traits::{Human, Cat, Animal};

pub fn vec_diff_types() {
    let mut creatures = Vec::new();
    //The following wont compile.. because, the compiler is gonna assume that the vec is of type Human.
    // creatures.push(Human{name: "John"});
    // creatures.push(Cat{name: "Fluffy"});

    //Well, there are 2distinct approaches to have different types in Vec.
    //1. enum
    creatures.push(Creature::Human(
        // Human { name: "John"}
        Human::create("John")
    ));
    creatures.push(Creature::Cat(
        // Cat { name: "Fluffy"}
        Cat::create("Fluffy")
    ));

    for c in creatures {
        //Well, here I cannot say,
        // c.talk()
        // because c is a Creature
        // So we could do something like
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(cat) => cat.talk(),
        }
    }

    // The downside.  why would I need to specify a new enum when I already have Animal trait.
    // Also, why would do pattern matching. for simple iterations.



    //2nd appraoach


    let mut animals:Vec<Box<dyn Animal>> = Vec::new();
    animals.push(
        Box::new(Human::create("John"))
    );
    animals.push(
        Box::new(Cat::create("Fluffy"))
    );

    for animal in animals.iter() {
        animal.talk();
    }
}


enum Creature {
    Human(Human),
    Cat(Cat)
}

