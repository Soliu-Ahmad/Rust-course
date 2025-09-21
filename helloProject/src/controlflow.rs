// IF..Else.. Conditional Statements
// (part of control flow)
// In Rust, conditional statements are used to execute different blocks of code based on certain conditions.
// The primary conditional statements in Rust are if, else if, and else.

// 1. conditions [if, else if, else]
// 2. loops [loop, while, for]
// 3. Repeat action until a condition is met [loop, while]
fn main() {
    let age: u16 = 20;

    if age >= 18 {
        println!("You can vote!");
    } else {
        println!("You cannot vote yet.");
    }

    // Multiple conditions with else if

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


   

    // else if
    // let number: i32 = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // // Using if in a let statement
    // let condition: bool = true;
    // let number: i32 = if condition { 5 } else { 6 };
    // println!("The value of number is: {}", number);

    // // Looping with loop, while, and for

    // // Infinite loop with break
    // let mut count: i32 = 0;
    // loop {
    //     count += 1;
    //     if count == 5 {
    //         break;
    //     }
    //     println!("Count is: {}", count);
    // }

    // // While loop
    // let mut number: i32 = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");

    // // For loop
    // let a: [i32; 5] = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("The value is: {}", element);
    // }

    // // For loop with range
    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

      // Using if in a let statement
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("Number: {number}");

   
}