use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn game() {
    println!();
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("{}", secret_number);

    loop {
        let mut guess = String::new();
        let a = io::stdin().read_line(&mut guess).expect("failed to read");

        // let guess: u32 = guess.trim().parse().expect("need number");
        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };
        println!("{}, input length -> {}", guess, a);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
            Ordering::Greater => println!("greater"),
        }
    }
}
