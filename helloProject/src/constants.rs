// Constants used throughout the project
// Constants are always immutable and must have a type annotation
// Constants can be declared in any scope, including the global scope
// Constants are valid for the entire duration of the program
// Constants are declared using the const keyword


const PI: f64 = 3.14159; // constant variable with type annotation
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant variable with type annotation

fn main() {
    println!("Hello, Rust from Constants!");
    const MAX_POINTS: u32 = 100_000; // constant variable
    println!("The maximum points are: {}", MAX_POINTS);

    let x = 5; // variable does not need to be mutable

    const Y: i32 = 10; // constant variable

    println!("The value of Y is: {}", Y);
    println!("The value of x is: {}", x);

    println!("The value of PI is: {}", PI);
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}

// you cant declear a constant variable inside a function
// const MAX_SCORE: u32 = 200; // Error! Constants cannot be declared inside
// the function body
// You can use shadowing to declare a new variable with the same name as a previous variable
// Shadowing is different from mutability
// Example:


// you can declear a constant with a type annotation    
// const PI: f64 = 3.14159; // constant variable with type annotation
// println!("The value of PI is: {}", PI);


