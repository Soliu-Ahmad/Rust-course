// // Functions
// // Entry point of the program
// // an function / variable should be written in snake case 
// // snake case: for example hello_world
// // rabab case: for example hello-world
// fn main() {
//     hello_world();
//     tell_height(182);
//     human_id(name: "Johe", age: 55 , height: 182.0);
// }

// // Hoisting can call function any where in ur code 

// fn hello_world(){
//     println!("Hello, Rust from Functions!");
// }

// // you can insert input values
// fn tell_height(height: u32){
//     println!("My height is {} cm.", height);
// }

// // You can insert more than one value 
// fn human_id(name: &str, age: u32, height: f32){
//     println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
// }

// Functions
// Entry point of the program
// a function / variable should be written in snake_case 

fn main() {
    hello_world();
    tell_height(182);
    human_id("Johe", 55, 182.0);   // ✅ Correct function call

    let calculate_total: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Result is: {}", calculate_total);
    // add(a: 4, b: 6)
    let y: i32 = add(4, 6);   // ✅ Fixed
    println!("Value of y is : {}", y);
    println!("Value from function 'add' is: {}.", add(4,6));
}

// Hoisting means you can call a function anywhere in your code

fn hello_world() {
    println!("Hello, Rust from Functions!");
}

// you can insert input values
fn tell_height(height: u32) {
    println!("My height is {} cm.", height);
}

// You can insert more than one value 
fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}


// Function returning values
// Function returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expression and Statements 
// Expression: Anything that returns a value.
// Statement: Anything that does not return a value.

// Expression
//------------
// true & false
// add(3, 4)
// if Conditions {value1} else {value2}
