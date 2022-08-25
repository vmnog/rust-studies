use std::io;

fn string_to_f64(string: &String) -> f64 {
    match string.trim().parse::<f64>() {
        Ok(num) => num,
        Err(err) => panic!("{err}"),
    }
}

fn fahrenheit_to_celsius(temperature: &f64) -> f64 {
    (temperature - 32_f64) * 0.5556_f64
}

fn main() {
    println!("Insert the temperature in Fahrenheit: ");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    println!("temperature is: {temperature}");
    
    let temperature: f64 = string_to_f64(&temperature);
    let converted_temperature: f64 = fahrenheit_to_celsius(&temperature);

    println!(
        "{}°F in Celsius is {}°C",
        temperature,
        converted_temperature.round()
    );
}
