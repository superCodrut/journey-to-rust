use std::mem;

struct Point {
    x:f64,
    y:f64
}

fn origin () -> Point {
    Point{x:0.0,y:0.0}
}


pub fn stack_and_heap() {

}

fn main() {
    println!("Hello, world!");
    let x = origin();// stack allocation
    let y = Box::new(origin());// heap
    stack_and_heap();
    println!("x takes {} bytes",mem::size_of_val(&x));
    println!("y takes {} bytes",mem::size_of_val(&y));

    let z = *y; // back on stack
    println!("{}",z.x);
}
