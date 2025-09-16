// Shadowing means you can declare a new variable with the same name as a previous variable
// Shadowing is different from mutability
// Example:
fn main(){
    let x: i32 = 5; // immutable variable
    println!("The value of x is: {}", x);
    let x: i32 = x + 1; // shadowing the previous x
    println!("The new value of x is: {}", x);
    {
        let x: i32 = x * 2; // shadowing the previous x in a new scope
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);
}