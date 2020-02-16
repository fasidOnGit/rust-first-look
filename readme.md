### First look at Rust
Simple start could be
`rustc ./hello.rs` and then `./hello` (executable created from the former command).

More simple way!

`cargo new name_of_the_project` and then `cargo build` and `cargo run`


Even more simple way! (this is getting intreseting).

IntelliJ to the rescue. Click the green triangle and there you go, we see
`"Hello Rust!`

##### Representation of Data:
* Boolean (true/false)
* Integers and decimal numbers
* Text
* Structured data (HTML, XML, JSON, etc..)
* Binary Data (images, proprietary formats)


##### Core Data types
```$xslt
// We take a single byte(8bit) and we put the number 123 with a memory representation `a`.
// u8: unsigned 0.... 255
let a:u8 = 123;
```

The values that are created in rust (say for ex `let a:u8 = 123`) are immutable. You can re-assign!
use **mut** keyword necessary to let know rust that the variable is mutable.
