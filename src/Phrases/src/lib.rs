pub mod greetings {
    pub mod english {
        pub fn hello() -> String {
            "hello".to_string()
        }
        pub fn goodbye() -> String {
            "goodbye".to_string()
        }
    }
    pub mod french {
        pub fn hello() -> String {
            "bonjour".to_string()
        }
        pub fn goodbye() -> String {
            "au revoir".to_string()
        }
    }
}

//UTs Simplified.
#[test]
fn english_greeting_correct() {
    assert_eq!("hello", greetings::english::hello());
}
#[test]
#[should_panic]
#[ignore]
fn english_greeting_incorrect() {
    assert_eq!("helloww", greetings::english::hello());
}
//there is more to test! like cargo tests...  google as needed. this video is idotic.. not loading!  bloody udemy
