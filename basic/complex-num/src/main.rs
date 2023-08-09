use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 5;
    x + y; 
}