use std::mem;

const MOL:u8 = 42; // no fixed address

static mol:i32 = 123;
static mut mol1:i32 = 123;


fn scope_and_shadowing() {

    let a = 123;

    {
        let b = 10;
        println!("inside, b = {}", b);
        println!("inside, a = {}", a);
        let a = b + 10;
        println!("inside1, a = {}", a);

    }


    println!("a = {}", a);
}

fn main() {
    println!("Hello, world!");
    scope_and_shadowing();
    println!("{}",MOL);
    println!("{}",mol);
    unsafe {
        mol1 = mol1 + 2;
        println!("{}",mol1);
    }
}
