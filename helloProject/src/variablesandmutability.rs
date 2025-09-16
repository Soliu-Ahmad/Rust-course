fn main(){
    println!("Hello, Rust from Variables and Mutability!");
    let mut a: i32 = 10; // immutable variable
    println!("The value of a is: {}", a);
    a = 20; // re-assigning a new value to a
    println!("The new value of a is: {}", a);
}

// Variables and Mutability
// Variables are immutable by default
// Use the mut keyword to make a variable mutable
// Constants are always immutable and must have a type annotation
// Shadowing allows you to declare a new variable with the same name as a previous variable
// Shadowing is different from mutability
// Example: