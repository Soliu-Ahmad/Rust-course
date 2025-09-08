// fn main() {
//     println!("Hello, Rust from CARGO!");
// }


// premitive data Types 
// int, float, bool, char

// Integer
//Rust has signed (+ and -) and Unsigned integer (only positive) types of different sizes.
// i8, i16, i32, i64, i128: signed integers.
// u8, u16, u32, u64, u128: unsigned integers.

fn main() {
    let x: i32 = 42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    
    // different between  i32 (2 bits) and i64(64 bits)
    // range :
    // i32: -2147483648 to 2147483647
    // i64: -9223372036854775808 to 9223372036854775807

    let e: i32 = 2147483647;
    let f: i64 = 9223372036854775807;

    println!("Signed Integer: {}", e);
    println!("Signed Integer: {}", f);



// Floating [floating point Types]
// f32, f64

let pi: f64 = 3.14;
println!("value of pi: {}", pi);

// boolean values: true or false
let is_valid: bool = true;
println!("is_valid: {}", is_valid);

// character type - char
let letter: char = 'a';
println!("first letter of the alphabet: {}", letter);


}

