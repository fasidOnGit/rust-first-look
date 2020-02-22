#![allow(unused_mut)]
#![allow(unused_variables)]

use std::ops::{Add, AddAssign, Neg};

#[derive(Debug)]
struct Complex<T> {
    real: T,
    imaginary: T
}

impl<T> Complex<T> {
    fn new(real: T, imaginary: T) -> Complex<T> {
        Complex::<T> {real, imaginary}  //:: is needed, or < is considered a less than operator.
    }
}

// impl Add for Complex<i32> {
//     type Output = Complex<i32>;
//
//     //a+b
//     fn add(self, rhs: Self) -> Self::Output {
//         Complex {
//             real: self.real + rhs.real,
//             imaginary: self.imaginary + rhs.imaginary
//         }
//     }
// }
pub fn op_overload() {
    let mut a: Complex<f64> = Complex::new(1.3, 2.5);
    let mut b = Complex::new(3.3, 4.3);
    // println!("{:?}", a + b);

    // a += b;
    // println!("{:?}", -a);

    //Equality
    println!("{:?}", a==b);
}
//Well the above code totally works. but we are limited to i32 impl of Complex for ADD operation.
// We are trying to make it worm for Complex<T> + Complex<T>

impl<T> Add for Complex<T>  where T: Add<Output = T>{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl<T> AddAssign for Complex<T> where T: AddAssign<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    }
}

impl<T> Neg for Complex<T> where T: Neg<Output = T> {
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary
        }
    }
}


//Partial Equality vs Full Equality..
// full eq: x=x
// But full eq is not the case with f32.
// because we NaN
// NaN == anything (including NaN) is false.. So this basically breaks the full eq here.
// So we need something like partial eq.

impl<T> PartialEq for Complex<T> where T: PartialEq {
    fn eq(&self, rhs: &Self) -> bool {
        self.real == rhs.real && self.imaginary == rhs.imaginary
    }
}

// Full Eq.
//Basically no implmentation is needed here. because it uses from PartialEq
impl<T: Eq> Eq for Complex<T> where T: Eq {}

// All these things are not necessary... you could just derive it at the top like we did for Debug,
// something like #[derive(Debug, PartialEq, Eq)]
