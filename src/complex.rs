#[derive(Clone)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    // TODO 1 - implement run
    fn new(_real: f64, _imaginary: f64) -> Complex {
        Complex {
            real: _real,
            imaginary: _imaginary,
        }
    }
}

use std::fmt;
use std::ops::{Add, Sub};

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let ans = match (self.real, self.imaginary) {
            (ref x , ref y) if *x == 0.0 && *y == 0.0 => write!(f, "0"),
            (ref x, ref y) if *y == 0.0 => write!(f, "{}", x),
            (ref x, ref y) if *x == 0.0 => write!(f, "{}i", y),
            (ref x, ref y) if *y < 0.0 => write!(f, "{}{}i", x, y),
            (ref z, ref t) => write!(f, "{}+{}i", z, t),
        };

        ans
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imaginary == other.imaginary
    }
}

fn module(x : &Complex) -> f64 {
    let ans = x.real * x.real + x.imaginary * x.imaginary;
    let ans = ans.sqrt();
    ans
}


impl PartialOrd for Complex {
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x = module(&self);
        let y = module(&other);
        x.partial_cmp(&y)
    }
    
    fn ge(&self, other: &Self) -> bool {
        module(&self) >= module(&other)
    }
    
    fn gt(&self, other: &Self) -> bool {
        module(&self) > module(&other)
    }

    fn le(&self, other: &Self) -> bool {
        module(&self) <= module(&other)
    }

    fn lt(&self, other: &Self) -> bool {
        module(&self) < module(&other)
    }
}

impl Copy for Complex {}


// TODO 2 - implement the Add, Sub, Mul and Div traits

impl Add for Complex {

    type Output = Self;
    fn add(self, other: Complex) -> Self::Output {
        let ans = Complex::new(self.real + other.real, self.imaginary + other.imaginary);
        ans
    }
}

impl Sub for Complex {

    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let ans = Complex::new(self.real - other.real, self.imaginary - other.imaginary);
        ans
    }
}

pub fn run() {
    let n1 = Complex::new(2 as f64, 3 as f64);
    let n2 = Complex::new(-2 as f64, 3 as f64);
    let n3 = Complex::new(2 as f64, -3 as f64);
    let n4 = Complex::new(3 as f64, 0 as f64);
    let n5 = Complex::new(0 as f64, 3 as f64);
    let n7 = Complex::new(2 as f64, 3 as f64);
    println!("The number is {}", n1); // prints 2+3i
    println!("The number is {}", n2); // prints -2+3i
    println!("The number is {}", n3); // prints 2-3i
    println!("The number is {}", n4); // prints 3
    println!("The number is {}", n5); // prints 3i

    println!("The number is {}", n1 - n7); // prints 0
    println!("The number is {}", n1 + n2);
    println!("The number is {}", n1 - n2);

    println!(
        "The numbers {} and {} are {}",
        n1,
        n2,
        if n1 == n2 { "equal" } else { "not equal" }
    );

    println!("The number is {}", n1 < n2);
    println!("The number is {}", n1 <= n2);
    println!("The number is {}", n1 > n2);
    println!("The number is {}", n1 >= n2);
}