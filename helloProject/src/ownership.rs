// Ownership, borrowing and Refrences
// Ownership
// C, C++ -> Memory Management Control Issues
// Rust -> Memory Safety without Garbage Collector
// 3 Rules of Ownership
// 1. Each value in Rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
// Scope is the range within a program where a variable is valid.
// Variable is valid from the point it is declared to the end of the block it is declared in.
// A block is a section of code that is enclosed in curly braces {}.
// When a variable goes out of scope, Rust calls the drop function and the memory is freed.
// Memory is automatically managed in Rust, so you don't have to worry about memory leaks or dangling pointers.
// Ownership can be transferred from one variable to another. This is called moving.
// When a variable is moved, the original variable is no longer valid.
// You can also borrow a variable using references. This is called borrowing.
// There are two types of references: immutable and mutable.
// Immutable references allow you to read the value without changing it.
// Mutable references allow you to change the value.
// You can have multiple immutable references to a value, but only one mutable reference.
// You cannot have a mutable reference while you have immutable references.
// This is called the borrowing rules.
// References are created using the & operator.
// Dereferencing is done using the * operator.  
// References do not own the value they point to, so they do not drop the value when they go out of scope.
// Garbage Collector (GC) is a form of automatic memory management.
// GC attempts to reclaim memory occupied by objects that are no longer in use by the program.
// Rust does not have a garbage collector. Instead, it uses a system of ownership with a set of rules that the compiler checks at compile time.
// This system ensures memory safety and prevents memory leaks


//1. Example: Each value in Rust has a variable that is called its owner.
fn main(){
    let s1 = String::from("Hello Rust");
    let len = calculate_length(&s1);  // Borrowing s1 using reference
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()  // Return the length of the String
} // No need to free memory, s1 is still valid here

//2. Example: There can only be one owner at a time.
fn main1() {
    let s1 = String::from("Rust");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // Error! s1 is no longer valid
    println!("{}", s2);  // This works
}

// 3. When the owner goes out of scope, the value will be dropped.
// This is demonstrated in the above examples. When s2 goes out of scope, the memory is freed automatically.
// Similarly, when s1 goes out of scope in main(), its memory is freed automatically.

fn main2() {
    let s = String::from("Hello");  // s is valid from this point
    let s1 = calculate_length(&s);
    println!("The length of '{}' is {}.", s, s1);
} // s goes out of scope and is dropped here


fn printlost(s: &String) {
    println!("{}", &s);
} // s goes out of scope here, but because it does not have ownership of what it refers to, nothing happens.

fn calculate_length1(s: &String) -> usize {  // s is a reference to a String
    s.len()  // Return the length of the String
} // No need to free memory, s1 is still valid here
// References do not own the value they point to, so they do not drop the value when they go out of scope.