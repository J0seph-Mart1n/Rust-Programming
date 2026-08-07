use std::io;

// To run this program, use the command: $ cargo run --bin fibonacci_number
fn main() {
    // Fibonacci number generator
    println!("Welcome to the Fibonacci Number Generator!");
    let mut n = 0;
    loop {
        println!("Enter the position of the Fibonacci number you want to generate (or type 'exit' to quit): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting the program.");
            break;
        }

        n = match input.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        let fib_number = fibonacci(n);
        println!("The Fibonacci number at position {} is: {}", n, fib_number);
    }
}

fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }

    b
}