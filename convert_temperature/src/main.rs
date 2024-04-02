use std::io;
const TEMPERATURES: [&str; 2] = ["Fahrenheit", "Celsius"];

fn main() {
    println!("👋 Welcome to 'Temperature Conversion!' Press ctrl+c to exit.");

    loop {
        println!("Select a temperature to convert to. Press 1 for Fahrenheit or 2 for Celsius.");

        let mut temperature_type = String::new();

        io::stdin()
            .read_line(&mut temperature_type)
            .expect("Failed to read your choice.");

        let temperature_type: usize = match temperature_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut temperature_type_message_modifier: &str = "";
        if temperature_type == 1 {
            temperature_type_message_modifier = TEMPERATURES[temperature_type];
        } else if temperature_type == 2 {
            temperature_type_message_modifier = TEMPERATURES[temperature_type - 2];
        }
        println!("Now enter the temperature in {temperature_type_message_modifier}.");

        let mut temperature_user_input = String::new();

        io::stdin()
            .read_line(&mut temperature_user_input)
            .expect("Failed to read your temperature input.");

        let temperature_user_input: f64 = match temperature_user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let converted_temperature: f64;

        if temperature_type == 1 {
            // Fahrenheit
            converted_temperature = temperature_user_input * 9.0 / 5.0 + 32.0
        } else {
            // Celsius
            converted_temperature = (temperature_user_input - 32.0) * 5.0 / 9.0
        }

        println!("{converted_temperature:.2}")
    }
}
