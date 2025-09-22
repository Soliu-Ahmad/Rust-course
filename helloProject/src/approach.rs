// approach 1
// enum Option<T> { // Define the generic Option type
//     Some(T), // Represents a value of type T
//     None, // Represents the absence of a value
// }

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// approach 2
// enum Result<T, E>{ // Define the generic Result type
//     Ok(T), // Represents a value
//     Err(E), // Represents an error
// }

fn divideOption(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    // Option approach
    let result: Option<f64> = divide(10.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by Zero!"),
    }

    // Result approach
    let result = divideResult(100.23, 73.98);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

// cannot be divided by 0