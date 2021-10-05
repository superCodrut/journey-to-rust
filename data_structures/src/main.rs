
use std::mem;

struct Point1<T>
{
    x:T,
    y:T
}

struct Line1<T>
{
    start: Point1<T>,
    end: Point1<T>
}


fn generics_test()
{
    let a = Point1 {x:2.3, y:1.3};
    let b = Point1 {x:1.2, y:32.2};

    let line = Line1 {start: a, end: b};
    println!("{:?}", line.start.x);
}

fn how_many_oranges (x:i32) -> &'static str
{
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        8...11 => "a lot",
        _ if x % 2 == 1 => "some",
        _ => " a few"
        
    }
}

fn match_test()
{
    for x in  0..13
    {
        println!("{}: I have {} oranges", x, how_many_oranges(x));
    }

    let point = (0, 2);
    match point
    {
        (0,0) => {println!("origin");},
        (x,0)=> {println!("y axis, x = {}", x);},
        (0,y) => {println!("x axis, y = {}", y);},
        (x,y) => {println!("x = {}, y = {}", x, y);}
    }
    let c:Color = Color::CmykColor {cyan:0, magenta:20, yellow:21, black:255};
    match c {
        Color::Red => {println!("r");}
        Color::Green => {println!("g");}
        Color::Blue => {println!("b");}
        Color::RgbColor(0,0,0)
        | Color::CmykColor{black:255,..} => {println!("black");}
        Color::RgbColor(r,g,b) => {println!("RgbColor ({},{},{})",r, g, b);}
        _ => ()
    }
}


fn sum_product (x:i32, y:i32) -> (i32, i32)
{
    (x+y, x*y)
}

fn tuples_test()
{
    let x = 4;
    let y = 5;

    let res = sum_product(x, y);
    println!("{0} + {1} = {2} and {0} * {1} = {3} ", x, y, res.0, res.1);

    let (a, b) = res;
    println!("s = {}, p = {}", a, b);
    println!("{:?}",res);

    let res2 = sum_product (10, 5);
    let comb = (res, res2);
    println!("{:?}", comb);
    println!("{}", (comb.1).1);
}

fn use_slice(slice: &mut[i32])
{
    println!("1st elem = {} and len = {}", slice[0], slice.len());
    slice[0] = 1244;
}

fn slice_test()
{
    let mut data = [1, 2, 3, 4, 5];

    //use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data);


}

fn array_test()
{
    let mut a:[i32; 5] = [1, 2, 3, 4, 5];

    println!("length of a is {} and 1st element is {}",
             a.len(), a[0]);
    println!("{:?}", a);
    let b = [3;10];
    for i in 0..b.len()
    {
        println!("{}",b[i]);
    }
    println!("b takes {} bytest", mem::size_of_val(&b));

    let mtx: [[f32;3]; 2] =
        [
            [1.0, 2.3, 1.9],
            [2.1, 3.3, 0.1]
        ];
    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("dig = {}",mtx[i][j]);
            }
        }
    }
}

fn option_test()
{
    let x = 3.0;
    let y = 1.0;

    let z = if y != 0.0 {Some (x/y)} else {None};

    match z {
        None => {println!("Canot divide by zero");},
        Some (w) => {println!("result = {}", w);}
    };

    if let Some(f) = z
    {
        println!("result1 = {}", f);
    }
}


enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor {cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn enum1()
{
    let c:Color = Color::CmykColor {cyan:0, magenta:20, yellow:21, black:255};
    match c {
        Color::Red => {println!("r");}
        Color::Green => {println!("g");}
        Color::Blue => {println!("b");}
        Color::RgbColor(0,0,0)
        | Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255} => {println!("black");}
        Color::RgbColor(r,g,b) => {println!("RgbColor ({},{},{})",r, g, b);}
        _ => ()
    }
}

struct Line {
    start:Point,
    end: Point
}


struct Point {
    x:f64,
    y:f64
}

fn struct_point ()
{
    let p = Point {x:3.0, y:4.0};
    println!("p is at ({}, {})", p.x, p.y);

    let p2 = Point {x:5.1, y:4.2};
    let line = Line {start:p, end:p2};
}

fn main() {
    println!("Hello, world!");

    generics_test();
    //match_test();
    //tuples_test();
    //slice_test();
    //array_test();
    //option_test();
    //enum1();
    //struct_point();
}
