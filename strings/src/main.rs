
use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Hello, world!");

    let number = rand::thread_rng().gen_range(1,101);
    loop
    {
        println!("Enter your guess:");
        let mut buff = String::new();
        match stdin().read_line(&mut buff)
        {
            Ok(_) => 
            {
                let parse = buff.trim_end().parse::<i64>();
                match parse
                {
                    Ok(x) =>
                    {
                        if x == number
                        {
                            println!("Correct");
                            break;
                        }else if x >100 || x < 1
                        {
                            println!("Your number is out of bound");
                        }
                        else if x > number
                        {
                            println!("Your number is too high");
                        }
                        else if x < number
                        {
                            println!("Your number is too low");
                        }

                    },
                    Err(e) =>
                    {
                        println!("error while parsing number {}", e);
                    }
                }
            },
            Err (_) => {
                continue;
            }
        }
    }
}
