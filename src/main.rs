use colored::*;
use std::io;

fn main() {
    loop {
        println!("Enter 0 for fahrenheit to celcius, or 1 for celcius to fahrenheit.");
        let input_from_user = read_input();
        println!("");

        if input_from_user == 0.0 {
            println!("Converting from fahrenheit to celcius");
            println!("Enter number of degrees fahrenheit that you want to convert to celcius.");

            let num_of_fahrenheit = read_input();
            let calculated_celcius = fahrenheit_to_celcius(num_of_fahrenheit);

            println!(
                "{} degrees fahrenheit is {} degrees celcius.",
                num_of_fahrenheit, calculated_celcius
            );

            break;
        } else if input_from_user == 1.0 {
            println!("Converting from celcius to fahrenheit");
            println!("Enter number of degrees celcius that you want to convert to fahrenheit.");

            let num_of_celcius = read_input();
            let calculated_fahrenheit = celcius_to_fahrenheit(num_of_celcius);

            println!(
                "{} degrees celcius is {} degrees fahrenheit.",
                num_of_celcius, calculated_fahrenheit
            );

            break;
        } else {
            println!("{}", "Invalid input. Please enter 0 or 1.".red());
        }
    }
}

// method for reading user input
fn read_input() -> f32 {
    loop {
        let mut input = String::new();
        println!("Enter a number:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return input;
    }
}

// method for converting celcius to fahrenheit
fn celcius_to_fahrenheit(input: f32) -> f32 {
    (input * 1.8) + 32.0
}

// method for converting fahrenheit to celcius
fn fahrenheit_to_celcius(input: f32) -> f32 {
    (input - 32.0) / 1.8
}
