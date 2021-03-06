#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;
use std::f64::consts::PI;
use crate::stack_heap::stack_and_heap;
use crate::control_flow::{if_statement, while_and_loop};
use crate::data_structure::{structures, enums};
use crate::pattern_matching::pattern_match;
use crate::generics::generic;
use crate::functions::print_value;
use crate::traits::traits_fun;
use crate::trait_as_parameters::trait_as_param_main;
use crate::operator_overloading::op_overload;
use crate::dispatch::dispatch_invocation;
use crate::vec_of_diff_types::vec_diff_types;
use crate::owenership::ownership_func;
use crate::circular_references::circular_references_func;
use crate::extern_crate::crates;

mod stack_heap;
mod control_flow;
mod data_structure;
mod pattern_matching;
mod generics;
mod functions;
mod traits;
mod trait_as_parameters;
mod operator_overloading;
mod dispatch;
mod vec_of_diff_types;
mod owenership;
mod circular_references;
mod extern_crate;

// type annotation is must here.!!
const MEANING_OF_LIFE:u8 = 42;  //  Gotcha, there is no fixed address.
// So if go ahead and print something like
//println!("{}", MEANING_OF_LIFE) //  this is as if I'had written 42. the value just replaces. there is no fixed address created.

static mut AA:i32 = 123;

fn main() {
    integral_types();
    bool_types();
    operators();
    scope_shadowing();
    unsafe {
        AA = 777;
        println!("{}", AA); // You are promising that you are not messing things up. which is why we specify `unsafe`
    }
    stack_and_heap();
    if_statement();
    while_and_loop();
    structures();
    enums();
    pattern_match();
    generic();
    print_value(33);
    traits_fun();
    trait_as_param_main();
    string_types();
    op_overload();
    dispatch_invocation();
    vec_diff_types();
    ownership_func();
    circular_references_func();
    crates();
}

fn operators() {
    //arithmetic
    let mut a = 2+3*4; // +-*/
    println!("{}", a);
    a = a+1; // -- ++  not supported in rust.
//    but this is supported
    a -= 2;  //that decreases
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64:: powf(b, PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise
    let c = 1 | 2; // | OR,  & AND, ^ XOR, ! NOR
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = PI < 4.0; //true
    // > <= >= ==
}

fn integral_types() {
    // We take a single byte(8bit) and we put the number 123 with a memory representation `a`.
    //
    let a:u8 = 123; //immutable
    println!("a = {}", a);

    let mut b:i8 = 0; //mutable
    println!("b = {}", b);
    b = 43;
    println!("b = {}", b);

    let mut c = 123456789;  // 32-but signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
//    i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123;  //isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} byte, {}-bit os", z, size_of_z, size_of_z * 8);

    //floats
    let e = 2.5; // becomes double-precesion, 8bytes or 64bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));
}

fn string_types() {
    let d: char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
}

fn bool_types() {
    let g = false; //bool
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn scope_shadowing() {
    let a = 123;
    {
        // Creates a scope.
        let b = 456;
        println!("inside b = {}", b);

        let a = 777;  // this is called shadowing.,
        println!("inside a = {}", a);
    }
    //b is no longer available here.
    println!("a = {}", a);
}
