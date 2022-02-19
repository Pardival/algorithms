use rand::prelude::*;
use std::io::*;

fn main() {
    let mut option = String::new();
    println!("++++++++++++++++++++++++++++++++");
    println!("+       Algorithms app         +");
    println!("++++++++++++++++++++++++++++++++");
    println!("+ Choose an option             +");
    println!("++++++++++++++++++++++++++++++++");
    println!("+ More or Less : A             +");
    println!("+ Fibonacci : B                +");
    println!("+ Fahrenheit : C               +");
    println!("++++++++++++++++++++++++++++++++");

    stdin().read_line(&mut option)
           .expect("Error when we read the user in");

    if option.trim() == "A" {
        more_or_less();
    } else if option.trim() == "B" {
        let mut n = String::new();

        println!("Write a number :");
        stdin().read_line(&mut n)
               .expect("Error when we read the user in");
        
        let n = n.trim()
                 .parse()
                 .expect("PLEASE, enter a number (positive)");

        println!("{}", fibonacci_generator(n));
    } else if option.trim() == "C" {
        let mut n = String::new();

        println!("Write a celcius number :");
        stdin().read_line(&mut n)
               .expect("Error when we read the user in");
        
        let n = n.trim()
                 .parse()
                 .expect("PLEASE, enter a number");

        println!("We have : {} Fahrenheit", celsius_to_fahrenheit(n));
    } else {
        println!("nothing good in cmd");
    }
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

fn fibonacci_generator(n: u32) -> u32 {
    let mut fibonacci1: u32 = 0;
    let mut fibonacci2: u32 = 1;

    if n == 0 {
        fibonacci1
    } else if n == 1 {
        fibonacci2
    } else {
        for _step in 1..n {
            let tempo = fibonacci2;
            fibonacci2 = fibonacci2 + fibonacci1;
            fibonacci1 = tempo;
        }
        fibonacci2
    }
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0/5.0 + 32.0
}