use rand::prelude::*;
use std::io::*;

fn main() {
    println!("Hello, world!");
    more_or_less();
}

/// More or less game
fn more_or_less() {
    const MAX_VALUE: i32 = 250;
    const MIN_VALUE: i32 = 0;

    let random_number: i32 = thread_rng().gen_range(MIN_VALUE..MAX_VALUE);
    
    println!("++++++++++++++++++++++++++++++++");
    println!("+        MORE OR LESS          +");
    println!("++++++++++++++++++++++++++++++++");

    loop {
        let mut value = String::new();

        println!("Write a number :");
        stdin().read_line(&mut value)
               .expect("Error when we read the user in");

        let value: i32 = value.trim()
                              .parse()
                              .expect("PLEASE, enter a number");

        if value >= MIN_VALUE && value <= MAX_VALUE {
            if value < random_number {
                println!("MORE");
            } else if value > random_number {
                println!("LESS");
            } else if value == random_number {
                println!("WIN !");
                break;
            }
        } else {
            println!("PLEASE, enter a number between {} and {}", MIN_VALUE, MAX_VALUE);
        }
    }
}