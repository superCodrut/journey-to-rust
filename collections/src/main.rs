
use std::collections::HashMap;
use std::collections::HashSet;


fn iterators()
{
    let mut vec = vec![3, 2, 1];
    println!("{:?}", vec);

    for x in &vec
    {
        println!("x = {}",*x);
        //x = x + 20;
        //println!("x = {}",*x);
    }

    for x in vec.iter_mut().rev()
    {
        println!("we got {}", *x);
        *x = 100 + *x;
        println!("x = {}",*x);
    }

    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec);
    println!("{:?}", vec2);
}

fn hashset_test()
{
    let mut hs = HashSet::new();

    hs.insert("r");
    hs.insert("g");
    println!("{:?}", hs);

    let status = hs.insert("r");
    if status == true
    {
        println!("Succes and hs = {:?}", hs);
    }
    if hs.contains("r")
    {
        println!("r is part of hs");
    }

    let rem = hs.remove("a");
    if !rem
    {
        println!("Failed to remove a");
    }

    let _1_10: HashSet<_> = (1..=10).collect();
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    //subset
    println!("is {:?} a subset of {:?} {}",
             _2_8, _1_10, _2_8.is_subset(&_1_10));

    //disjoint
    println!("is {:?} disjoint {:?} {}",
             _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    // union
    println!("union of {:?} with {:?} is {:?}",
             _2_8, _6_10, _2_8.union(&_6_10));

    // intersaction
    println!("intersaction between {:?} and {:?} is {:?}",
             _1_5, _1_10, _1_5.intersection(&_1_10) );
}

fn hashmap_test()
{

    let mut shapes = HashMap::new();

    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["triangle".into()]);
    for (k, v) in & shapes
    {
        println!("k = {}, v = {}", k, v);
    }
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(10);
        *actual = -3;
    }
    println!("{:?}", shapes);
}

fn vec_test()
{
    let mut a = Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}",a);
    a.push(44);
    println!("a = {:?}",a);
    


    let idx:usize = 2;
    match a.get(idx) {
        Some(w) => {println!("at pos {} the val is {}", idx, w);},
        None => {println!("wrong");}
    }

    for x in &a {println!("{}", x);}

    a.push(77);
    let last = a.pop();
    // Option
    println!("last = {:?}, a = {:?}", last, a);


    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
}

fn main() {
    println!("Hello, world!");
    iterators();
    //hashset_test();
//    hashmap_test();

    //vec_test();
}
