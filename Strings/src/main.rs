fn main() {
    let greeting: String = String::from("Hello, world!");
    // Another way to declare a string literal is to use a string slice (&str) instead of a String object.
    // let greeting: &str = "Hello, world!";
    println!("{}", greeting);

    let char1: Option<char> = greeting.chars().nth(1000);

    match char1 {
        Some(c) => println!("The first character of the greeting is: {}", c),
        None => println!("The character is out of bound."),
    }

    println!("{:?}", char1);
}
