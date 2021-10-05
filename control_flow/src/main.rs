use std::io::stdin;



 enum State
{
    Locked,
    Failed,
    Unlocked
}


fn combination_lock ()
{
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop 
    {
        match state
        {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    },
                    Err(_) => {continue;}
                }
                if entry == code { state = State::Unlocked; continue;}
                if !code.starts_with(&entry) {state = State::Failed;}

            },
            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;
            },
            State::Unlocked => {
                println!("Unlocked");
                return;
            }
        };
    }
}


fn match_test ()
{
    let code = 4;
    let country_code = match code {
        44 => "uk",
        4 => "Ro",
        1..=1000 => "unknown",
        _  => "invalid"
    };
    println!("code = {} => country = {}", code, country_code);
}


fn if_statement() {
    let temp = 14;

    if temp > 30
    {
        println!("hot");
    } else if temp < 10
    {
        println!("cold");
    } else
    {
        println!("ok");
    }
    let x = if temp > 20 {"sunny"} else {"cloudly"};
    println!("{}", x);
}

fn while_loop()
{
    let mut x = 1;

    while x < 1000
    {
        x *= 2;
        if x ==64 || x == 128 {continue;}
        println!("x = {}",x);
    }

    let mut y = 1;
    loop // while true
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10 {break;}
    }
}

fn for_loop ()
{
    for x in 1..11
    {
        if x == 3 {continue;}
        if x == 8 {break;}
        println!("x = {}",x);
        
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("pos {} = val {}",pos, y);
    }
}

fn main() {
    println!("Hello, world!");
    //if_statement();
    //while_loop();
//    for_loop();
    //match_test();
    combination_lock();
}
