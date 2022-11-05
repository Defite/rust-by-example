use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius and against");

    let read_input = move |msg: &str| {
        println!("{}", msg);
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    };

    let temperature: f32 = read_input("Enter temperature")
        .parse()
        .expect("Failed to enter number");

    let conversion_type: &str = &read_input("Choose unit Fahrenheit(F) or Celcius(C)");

    println!(
        "Your temperature: {}",
        format!("{:.1}", conversion_choice(temperature, conversion_type))
    );
}

fn fahrenheit_to_celsius(temperature: f32) -> f32 {
    return (temperature - 32.0) * 5.0 / 9.0;
}

fn celsius_to_fahrenheit(temperature: f32) -> f32 {
    return (temperature * 9.0 / 5.0) + 32.0;
}

fn conversion_choice(temperature: f32, conversion_type: &str) -> f32 {
    match conversion_type {
        "F" => fahrenheit_to_celsius(temperature),
        "C" => celsius_to_fahrenheit(temperature),
        _ => 0.0,
    }
}
