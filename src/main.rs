use std::io;

fn to_kelvin(input: u32, temp: &f32) -> f32 {
    if input == 1 {
        let kel: f32 = temp + 273.15;
        kel
    } else {
        let kel: f32 = (temp - 32.0) / 1.8 + 273.15;
        kel
    }

}

fn to_fahrenheit(input: u32, temp: &f32) -> f32 {
    if input == 1 {
        let fah: f32 = (temp * 1.8) + 32.0;
        fah
    } else {
        let fah: f32 = (temp - 273.15) * 9.0 / 5.0 + 32.0;
        fah
    }

}

fn to_celsius(input: u32, temp: &f32) -> f32 {
    if input == 2 {
        let cel: f32 = temp - 273.15;
        cel
    } else {
        let cel: f32 = (temp - 32.0) * 5.0 / 9.0;
        cel
    }

}

fn main() {
    let mut input: String = String::new();
    println!("Hello, This program converts between Celcius, Kelvin and Fahrenheit");
    println!("Input which temp value your are going to input");
    println!("1. Celcius");
    println!("2. Kelvin");
    println!("3. Fahrenheit");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("You inputted: {}", input);

    let input: u32 = input.trim().parse().expect("Type a number");
    let mut temp: String = String::new();
    println!("Input temperature for conversion:");

    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp: f32 = temp.trim().parse().expect("type a decimal number");

    match input {
        1 => { 
            let kel = to_kelvin(input, &temp);
            let fah = to_fahrenheit(input, &temp);
            println!("{} Celsius is", temp);
            println!("Kelvin: {:.2}", kel);
            println!("Fahrenheit: {:.2}", fah);
        },
        2 => {
            let cel = to_celsius(input, &temp);
            let fah = to_fahrenheit(input, &temp);
            println!("{} Kelvin is", temp);
            println!("Celsius: {:.2}", cel);
            println!("Fahrenheit: {:.2}", fah)
        }
        3 => {
            let cel = to_celsius(input, &temp);
            let kel = to_kelvin(input, &temp);
            println!("{} Fahrenheit is", temp);
            println!("Celsius: {:.2}", cel);
            println!("Kelvin: {:.2}", kel);
        },
        _ => println!("You entered unprocessable number {} for temp type", input),
    }

}
