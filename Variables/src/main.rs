fn main() {
    // mut keyword is used to make a variable mutable, allowing its value to be changed after it is initially set.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing allows you to declare a new variable with the same name as a previous variable, effectively "shadowing" the previous variable. This can be useful for transforming a value while keeping the same variable name.
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}