use std::mem;

fn main() {
    // u -> unsinged 0...2^N - 1
    let a: u8 = 123; // immutable by default
    println!("A = {}", a);

    // i -> signed -2^(N-1) ... 2^ (N0-1) - 1
    let mut b: i8 = 10; // mutable
    println!("b = {}", b);
    b = b - 20;
    println!("b = {}", b);

    let c = 123456789; // i32
    println!("c = {}, takes up to {} bytes.", c, mem::size_of_val(&c));


    //usize and isize
    let d: usize = 123;
    let mut e: isize = -123;
    println!("d = {}, e = {}", d, e);
    e = e - 12;
    println!("e = {}", e);
    let size_of_d = mem::size_of_val(&d);
    let size_of_e = mem::size_of_val(&e);
    println!("sizeof D = {}, sizeof E = {}", size_of_d, size_of_e);

    let x: char = 'X';
    println!("X = {}, sizeof X = {}", x, mem::size_of_val(&x));

    let e1: f32 = 2.3;
    println!("e = {}, e1 has {} bytes", e1,mem::size_of_val(&e1));


    let f1: f64 = 4.1; // default
    println!("f1 = {}, f1 has {} bytes", f1, mem::size_of_val(&f1));

    let g: bool = true;
    println!("g = {}, g has {} bytes",g, mem::size_of_val(&g));
}
