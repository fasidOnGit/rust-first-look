use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

//The key selling point of rust is Memory safety.
pub fn ownership_func() {
    let v = vec![1,2,3]; // variable v kinda owns what is in the vector.
    // the variable v is on the Stack! but the vector on the other hand is in the heap.
    //Only a signal variable can own a memory at any given time.

    // we could say something like, in ordinary situation
    // we could assume that, we have two variables pointing to the same memory.
    // WEll. RUST is smart here. because here we have a race condition.
    let v2 = v;

    // println!("{:?}", v); // this wont compile. Why?
    //Because , at any given point a resource can be pointed by one variable.
    // So what rust does here is that, it invalidates v.
    // and gave away the handle to v2.
    // so v is no longer usable.
    // alright! But...
    let u = 1;
    let u2 = u;
    //But this works/compiles. in other words! it works for primitive types!
    // Well, what happens here is that we perform a full copy!
    // not moving!
    // this is because, i32 it's just a 32but chunk of memory allocated somewhere, so it's rather cheap to perform a full copy.
    // whereas in vector, it's pointer to a resource in the memory.
    // so there is no copy happening.
    // well we could implement Copy trait to do so and make it work. but that's not the point! :D

    // Thumb rule => when you have a variable pointing to something on the heap, then we can't copy!

    // Well, you would be tempted to write something like the below.
    let print_vec = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    let vv = print_vec(v2);
    println!("{}", vv[0]);

    //Ok, what essentially is happening here is that, the closure print_vec is returning the x.
    //So, it uses the Vec, and then again it's returning the control over it!. and it works as expected.

    borrowing();
}

fn borrowing() {
    // All the time we can't just use the closure to return the ownership.. So one other way of doing it is Borrowing.
    let v = vec![3,2,1];
    let print_vector = |x: &Vec<i32>| {
        println!("x[0] = {}", x[0]);
    };
    print_vector(&v);
    // here, with & we say, we lend v to print_vector.
    // print_vector borrows v. by taking a reference.
    // again x is immuutable reference.
    // so if we do x.push inside print_vector. it wouldn't compile because x is immutable.
    // and now it's totally lefgal to say
    println!("v[0] = {}", v[0]); // because v is not invalidated. it was just borrwed and everything would just works,

    // let talks mut
    let mut a = 40;
    let b = &mut a;  // b is a borrowed reference of a.
    // so to access the value of the refrence we say something like
    *b += 2;  // basically the * is to access the value that ref is referring to.
    println!("{}", a);

    // let mut z = vec![3,2,1];
    // for i in &z {
    //     println!("i = {}", i);
    //     z.push(*i);
    // }
    // This wouldn't work, because, trying to add the array inside of it gives you undefined behaviour.
    // but , rust won't allow you to do it.

    lifetimes();
}

fn lifetimes() {
    {
        let i = 3; // Lifetime for `i` starts. ────────────────┐
        //                                                     │
        { //                                                   │
            let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
            //                                                ││
            println!("borrow1: {}", borrow1); //              ││
        } // `borrow1 ends. ──────────────────────────────────┘│
        //                                                     │
        //                                                     │
        { //                                                   │
            let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
            //                                                ││
            println!("borrow2: {}", borrow2); //              ││
        } // `borrow2` ends. ─────────────────────────────────┘│
        //                                                     │
    } // Lifetime ends.
    failed_borrow();
    // &'static str  //essentially, static is the lifetime of the variable. static means it lives as long as the program lives.
    // so the part that defines the life time is this part. `'[lifetime]`. static is a special name but we can also have our own names.
    struct Person {
        name: String
    }
    //Well, the &Person expects a lifetime specifier
    // to do that in a struct. we say, it lives as long as the Company lives.
    struct Company<'z> {
        name: String,
        ceo: &'z Person
    }

    let boss = Person { name: "John".to_string()};
    let tesla = Company { name: "Tesla".to_string(), ceo: &boss};
    println!("{:?}", tesla.name);

    let four = 4;
    print_refs(&four);
    //Ok, let's say something like,
    impl Person {
        fn get_ref_name(&self) -> &String {
            &self.name
        }
    }
    //
    // let mut z: &String;
    // {
    //     let p = Person { name: "John".to_string()};
    //     z = p.get_ref_name(); // Guess what this won't compile!
        // p is short lived! , z has a longer life time.
        // Remember!, the lender must outlive the borrower! here the
        // lender is p which is short lived that z. hence wont compile.
        // And this applies only to borrowing! not when you hand off control of memory itself.
    // }
    // println!("{}", z);
    lifetimes_struct();
}

fn failed_borrow<'a>() {
    let _x = 12;
    // let y: &'a i32 = & _x;  // Ok this fails.
}

// Here is the thumb rule.
// Any input which is borrowed must outlive the borrower.

// So, in the failed_borrow funtion, we have a explicit annotation 'a , well, there is no reference to any variable that enforces 'a lifetime
// which means it defaults to 'static.  well, then it's longer, and lives untill the program ends.
// which means. the borrower y which has the lifetime as 'a has the longest lifetime. while the _x is shortlived to the function scope itself.

// Yes Liftimes and Scopes are not the same. there is this difference.
//let's take an example of where it works!

fn print_refs<'a>(x: &'a i32) {
    println!("{}", x);
}

fn lifetimes_struct() {

    // Need to specify lifetime specifier for both struct and implementation.
    struct Person<'a> {
        name: &'a str
    }
    impl<'a> Person<'a> {
        fn talk(&self) {
            println!("Hi, my name is {}", self.name)
        }
    }
    let person = Person { name: "Kader"};
    person.talk();

    rc_demo();
}

//Reference Counter Variable

fn rc_demo() {
    struct Person {
        name: String
    }
    impl Person {
        fn new(name: String) -> Person {
            Person { name }
        }

        fn greet(&self) {
            println!("Hello, Greetings!, my name is {}", self.name);
        }
    }

    // Won't compile because we are using a moved value here.
    //name is passed in person and also we are trying to print it!
    // let name = "John".to_string();
    // let person = Person::new(name);
    // person.greet();
    // println!("Name = {}", name);

    // Solution! RC!
    struct PersonRc {
        name: Rc<String>
    }

    impl PersonRc {
        fn new(name: Rc<String>) -> PersonRc {
            PersonRc { name }
        }
        fn talk(&self) {
            println!("Hello  as RC!, my name is {}", self.name);
        }
    }

    let name = Rc::new(String::from("Kader"));
    let person = PersonRc { name: name.clone() };
    person.talk();
    println!("Using it again! {}", name);
    println!("{} strong pointers", Rc::strong_count(&name));

    // Unfortunately, RC is limited to one thread.! So, if we want something that are thread safe. then,
    // Atomic Referenced counter variable!
    arc_demo();

}

fn arc_demo() {
    struct PersonArc {
        name: Arc<String>
    }

    impl PersonArc {
        fn new(name: Arc<String>) -> PersonArc {
            PersonArc { name }
        }
        fn talk(&self) {
            println!("Hello  as ARC!, my name is {}", self.name);
        }
    }
    let name = Arc::new("John".to_string());
    let person = PersonArc::new(name.clone());

    let t = thread::spawn(move || {
        person.talk();
    });
    println!("Name = {}", name);
    t.join().unwrap();
    // Ok what's the problem this has?
    // Well, Arc is really cool, it lets different threads to use a Rc variable.
    // But, it doesn't gaurd the variable from mutating by various threads.
    // So all we need is Mutex. meaning Mutaul Exclusion.
    // At it's core, we'd just lock a variable from mutating until the thread that locked the variable releases it
    mutex_demo();
}

fn mutex_demo() {
    struct PersonArcMutex {
        name: Arc<String>,
        state: Arc<Mutex<String>>
    }

    impl PersonArcMutex {
        fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> PersonArcMutex {
            PersonArcMutex { name , state }
        }
        fn greet(&self) {
            let mut state = self.state.lock().unwrap();
            state.clear();
            state.push_str("exicted!");
            println!("Hello  as ARC!, my name is {} and I am {}", self.name, state.as_str());
        }
    }
    let name = Arc::new("John".to_string());
    let state = Arc::new(
        Mutex::new("bored".to_string())
    );
    let person = PersonArcMutex::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {} state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();
}
