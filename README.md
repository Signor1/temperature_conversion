# Temperature Conversion Program

A simple Rust program that converts temperatures between Fahrenheit and Celsius. The program takes user input and allows for easy temperature conversions in both directions.

## Features

- **Conversion Modes**: 
  - Convert Fahrenheit to Celsius.
  - Convert Celsius to Fahrenheit.
- **User-Friendly Interface**: 
  - The program prompts the user to choose a conversion mode and enter the temperature value.
  - It validates user input, ensuring only valid numbers are accepted.

## Installation

Ensure you have Rust installed. If not, you can install it using [rustup](https://rustup.rs/).

## How to Run

Clone the repository, navigate to the project directory, and run:

```bash
cargo run
```

Follow the instructions in the terminal:
- Enter `0` to convert from Fahrenheit to Celsius.
- Enter `1` to convert from Celsius to Fahrenheit.

The program will ask for the temperature value and display the converted result.

## Example Usage

```
Enter 0 for fahrenheit to celcius, or 1 for celcius to fahrenheit.
Enter a number:
0

Converting from fahrenheit to celcius
Enter number of degrees fahrenheit that you want to convert to celcius.
Enter a number:
100
100 degrees fahrenheit is 37.77778 degrees celcius.
```

If an invalid option is selected:

```
Invalid input. Please enter 0 or 1.
```

## Dependencies

- **Standard Library**: The program uses Rust's standard library for reading user input.

## Contributing

Contributions are welcome! Fork the repository and create a pull request for any enhancements or additional features.
