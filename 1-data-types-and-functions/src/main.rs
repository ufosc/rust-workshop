// Example 1: Data Types and Functions

// This macro will make the compiler ignore warnings about unused variables.
// Typically it is only used in examples.
#![allow(unused)]

use std::io;

// Counts the spaces in a given string.
// Params: &str
// Returns: u32
fn count_spaces(line: &str) -> u32 { 
    let mut count: u32 = 0; // Initialize the counter as a u32 to match type
    
    for c in line.chars() {
        if c == ' ' { count += 1; }
    }

    count // If the last statement of a function doesn't end with ";", it will return it.
}

fn main() {
    // # Primatives
    // Integers
    let small_int: i8 = 2; // 8 bit signed integer
    let large_int: i128 = 100_000_000; // 128 bit signed integer
    let arch_int: isize = 64; // Architecture defined integer (Probably 64-bit)
    
    // Floats
    let small_float: f32 = 2.0; // 32-bit float 
    let large_float: f64 = 2.0000000006; // 64-bit float 

    // Booleans
    let t_bool: bool = true;
    let f_bool: bool = false;

    // Chars - Use 4 Bytes for UTF-8 Encoding!
    let e_char: char = 'E'; 
    let emoji_char: char = '🚀';

    // # Compound Types
    // Tuples
    let random_chars: (char, char, char, char) = ('G', 'M', e_char, emoji_char);
    let (w, x, y, z) = random_chars; // Destructuring the tuple
    let third_value: char = random_chars.2;

    // Arrays
    let array = [1, 2, 3]; // Allocated to the stack
    let zero_array = [0; 100]; // Initialize an array of size 100 with 0s 


    // # Strings -- 2 Types
    // String - Vector of bytes (Vec[u8]), guaranteed to be valid UTF-8. Heap allocated, growable, and not null terminated.
    // &str - Immutable sequence of UTF-8 bytes, guaranteed to be valid UTF-8. Not necessarily on the Heap. Typically used as a window to data.

    // Print a given String
    let mut name_string = "Tommy".to_string();
    name_string += " Wiseau";
    println!("Hello! My name is {}. What's your name?", name_string);

    // Collect the user's name
    let mut your_name_string = String::new();
    io::stdin()
        .read_line(&mut your_name_string)
        .expect("Failed to read line."); // Handle a possible exception

    // Get rid of the new line character from the input
    your_name_string = your_name_string.trim().to_string();

    // Greet the user
    println!("\nOh hi, {}. So how's your &str life?", your_name_string);

    // Functions
    // Collect a sentence from the user
    println!("Please input a sentence, {}.", your_name_string);
    let mut to_count_string = String::new();
    io::stdin()
        .read_line(&mut to_count_string)
        .expect("Failed to read line.");

   // Parse the string for spaces
   let spaces = count_spaces(&to_count_string);
   println!("\nYour sentence had {} spaces. Great job, {}.", spaces, your_name_string) 

}
