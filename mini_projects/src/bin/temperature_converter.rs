use std::io;

//To run this program, use the command: $ cargo run --bin temperature_converter
fn main() {
    // Fahrenheit to Celsius converter
    println!("Welcome to the Temperature Converter!");
    loop {
        println!("Choose an option:");
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => convert_fahrenheit_to_celsius(),
            2 => convert_celsius_to_fahrenheit(),
            3 => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    }
}

fn convert_fahrenheit_to_celsius() {
    println!("Enter the temperature in Fahrenheit: ");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read input");
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Enter a valid input");
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("The temperature in Celsius is: {:.2} C", celsius);
}

fn convert_celsius_to_fahrenheit() {
    println!("Enter the temperature in Celsius: ");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read input");
    let celsius: f64 = celsius.trim().parse().expect("Enter a valid input");
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("The temperature in Fahrenheit is: {:.2} F", fahrenheit);
}