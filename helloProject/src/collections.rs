// Collections Types
// Vectors - UTF8 - Hashmaps

fn main() {
    let _v: Vec<i32> = Vec::new();

    // with macro 
    let mut _the_vec: Vec<i32> = vec![1, 2, 3];
    
    let mut _the_numbers_: Vec<i32> = Vec::new();
    _the_numbers_.push(1);
    _the_numbers_.push(2);
    _the_numbers_.push(3);
    _the_numbers_.push(4);
    _the_numbers_.push(5);

    println!("The Vector is: {:?}", _the_numbers_);

    let _v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let Third: &i32 = &_v[2]; // Direct indexing    
    println!("The third element is: {}", Third);

    let third = _v.get(4); // Using get method
    match third {
        Some(third) => println!("The third element for a GET method is: {}", third),
        None => println!("There is no third element."), 
}

}