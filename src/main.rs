use std::io;
use unit_conversions::*;

fn main() {
    println!("This is temperature converter!");
    println!("Input temperature you want to convert.");

    let mut temperature_input = String::new();

    io::stdin()
        .read_line(&mut temperature_input)
        .expect("Failed to read line.");

    let temperature_input: i128 = temperature_input
        .trim()
        .parse()
        .expect("Please type a number!");

    println!(
        "
        Select the type of conversion.
        1, Fahrenheit to Celsius
        2, Celsius to Fahrenheit
        "
    );

    let mut selected_conversion = String::new();

    io::stdin()
        .read_line(&mut selected_conversion)
        .expect("Failed to read line");

    let selected_conversion: i128 = selected_conversion
        .trim()
        .parse()
        .expect("Please type a number!");

    let conversion_type = (1, 2);

    if selected_conversion == conversion_type.0 {
        let temperature_input: f64 = temperature_input as f64;
        let result = convert_to_c(temperature_input);
        println!("{}°F is {}°C in celsius!", temperature_input, result);
    } else {
        let temperature_input: f64 = temperature_input as f64;
        let result = convert_to_f(temperature_input);
        println!("{}°C is {}°F in fahrenheit!", temperature_input, result);
    }
}

fn convert_to_c(input: f64) -> i128 {
    let response = temperature::fahrenheit::to_celsius(input);
    let response = response as i128;
    response
}

fn convert_to_f(input: f64) -> i128 {
    let response = temperature::celsius::to_fahrenheit(input);
    let response = response as i128;
    response
}
