fn main() {
    your_name("Random", "Person");

    let sum = add_numbers(5, 10);
    println!("The sum of 5 and 10 is {sum}");
}

// Function with parameters
fn your_name(first_name: &str, last_name: &str) {
    println!("Your name is {first_name} {last_name}");
}

// Functions with Return Value
fn add_numbers(num1: i32, num2: i32) -> i32 { // Return type needs to be specified if the function returns a value
    return num1 + num2
}