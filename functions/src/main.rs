

fn is_even (x:u32) -> bool
{
    x % 2 == 0
}

fn greater_than_limit (limit: u32)
    -> impl Fn(u32) -> bool
{
    move |x| x > limit
}


fn hof ()
{
    // functions that take functions
    // sum of all even squares < 500
   let limit = 500;
   let mut sum = 0;
   let mut x = 1;

   let above_limit = greater_than_limit(limit);

   loop
   {
        let sq = x * x;
        if above_limit(sq)
        {
            break;
        }
        if is_even(sq)
        {
            sum += sq;
        }
        x += 1;
   }
   println!("sum = {}", sum); 
    // functions that return functions
    
    

    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|x: &u32| is_even(*x) )
        .fold(0, |sum, x| sum + x)
        ;
    println!("sum2 = {:?}", sum2);
}


fn closures()
{
    let plus_one = |x:i32| -> i32 {x+1};
    let a = 10;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    
    let plus_2 = |x|
    {
        let mut y = x;
        y += two;
        y
    };
    println!("{} + 2 = {}", a, plus_2(a));
    
    let mut f =12;
    let plus_3 = |x:&mut i32| {*x += 3};
    plus_3(&mut f);
    println!("f  = {}", f);
    
}


struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn methods()
{
    let p1 = Point {x:0.0, y:0.0};
    let p2 = Point {x:3.0, y:4.0};
    let line = Line {start:p1, end:p2};

    println!("Len = {}", line.len());
}


fn increase (x: &mut i32)
{
    *x += 1;
}

fn product (x: i32, y: i32) -> i32
{
    x * y
}

fn call_funct()
{
    let mut x = 1;
    println!("x = {}", x);
    increase(&mut x);
    println!("x = {}", x);
    let a = 5;
    let b = 3;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

fn main() {
    println!("Hello, world!");
    //call_funct();
    //methods();
    //closures();
    hof();
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_increase()
    {
        let mut x = 1;
        increase (&mut x);
        assert_eq!(2, x);
    }

    #[test]
    fn test_product()
    {
        assert_eq!(20, product(4, 5));
    }

    #[test]
    fn test_len_of_line()
    {

        let p1 = Point {x:0.0, y:0.0};
        let p2 = Point {x:3.0, y:4.0};
        let line = Line {start:p1, end:p2};
        assert_eq!(5.0, line.len());
    }
}
