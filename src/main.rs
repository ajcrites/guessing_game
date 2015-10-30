extern crate rand;
extern crate getopts;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use getopts::Options;
use std::env;

// Make the guess -- we only want numbers
// So keep prompting the user if they don't provide one
fn make_guess() -> u32 {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
    }
}

fn main() {
    println!("Guess the number!");

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag("", "debug", "print out debug information");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // --debug flag lets us know what the secret number is
    if matches.opt_present("debug") {
        println!("The secret number is: {}", secret_number);
    }

    loop {
        println!("Please input your guess.");

        let guess: u32 = make_guess();

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
