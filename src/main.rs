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
    println!("+ Insertion sort : D           +");
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
    } else if option.trim() == "D" {
        let mut simple_array: [isize; 9] = [4, -1, 10, 0, 0, 3, 20, 1, -3];

        insertion_sort(&mut simple_array);
        to_string(&simple_array);
        
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

/// Slow sort (insertion sort)
fn insertion_sort(to_sort: &mut [isize]) {
    for i in 0..to_sort.len() {
        let mut i_min: usize = i;
        let tempo: isize;
        for c in i+1..to_sort.len() {
            if to_sort[c] < to_sort[i_min] {
                i_min = c;
            }
        }
        tempo = to_sort[i];
        to_sort[i] = to_sort[i_min];
        to_sort[i_min] = tempo;
    }
}

fn to_string(array: &[isize]) {
    for i in 0..array.len() {
        println!("{} ", array[i]);
    }
}