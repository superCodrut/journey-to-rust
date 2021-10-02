use std::mem;


fn operators ()
{
 let mut  a = 2 + 3*3;
 println!("{}", a);
 a = a+1; // -- and ++ are not supported
 println!("{}", a);
 a -= 2;
 println!("{}", a);

 println!("{} % {} = {}", a, 3, (a%3));

 let a_cube = i32::pow(a,3);
 println!("{}", a_cube);

 let b = 2.5;
 println!(" {} ^ 3 = {}", b, f64::powi(b, 3));
 println!(" {} ^ PI = {}",b, f64::powf(b, std::f64::consts::PI));


 //bitwise ops only for integers
 // | = OR; & = AND; ^ = XOR; ! = NOR
 let x = 1 | 2;
 println!("x should be 3? = {}", x);
 let x_pow_10 = 1 << 10;
 println!("2^10 = {}", x_pow_10 );


 //logical operators:
 let pi_less_four = std::f64::consts::PI < 4.0;
 println!("{}",pi_less_four);

 let x = 5;
 let x_equl_5 = x == 5;
 println!("{}", x_equl_5);

}

fn main() {
    println!("Hello, world!");
    operators();

}
