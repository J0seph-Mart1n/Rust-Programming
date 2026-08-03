fn main() {
    // Integer Data Type
    let guess: u32 = 42; // Unsigned 32-bit integer
    let guess: i32 = -42; // Signed 32-bit integer

    // Floating-Point Data Type
    let pi: f32 = 3.14; // 32-bit floating-point number

    // Boolean Data Type
    let is_active: bool = true;

    // Character Data Type
    let letter: char = 'A';

    // Compound Types
    // Tuple Data Type
    let tuple: (i32, f64, u8) = (500, 6.4, 1); // Tuple can store multiple values of different types, Tuples have a fixed length.
    let (x, y, z) = tuple; // Getting individual values from the tuple
    let first_value = tuple.0; // You can use period notation to access individual values in a tuple.
    println!("The value of y is: {y}");

    // Array Data Type
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Array can store multiple values of the same type, Arrays have a fixed length.
    let a = [3; 5]; // This creates an array of length 5, where each element is initialized to the value 3. [3,3,3,3,3]
    println!("The value of the first element is: {}", array[0]);
}
