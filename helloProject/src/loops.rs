// Repetition with loops
fn main() {
    // Infinite loop with break
    let mut count = 0;
    
    let result = loop {
        count += 1;
        if count == 20 {
            break count - 100; // returning value from loop
        }
    };
    println!("The result is {}", result);
    // Loop labels to disambiguate between multiple nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;

    }

    println!("End count = {count}");

    // While loop
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
println!("LIFTOFF!!!");

// Looping through a collection with for loop
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("The value is: {element}");

}

}