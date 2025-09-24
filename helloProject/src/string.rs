// String is a collection of characters that is used to represent text. In Rust, strings are represented by the String type, which is a growable, heap-allocated data structure.
// Strings are UTF-8 encoded, which means they can represent a wide range of characters from different languages and scripts.
// Strings are mutable, which means you can change their contents after they are created.
// You can create a String using the String::new() function or by using the to_string() method on a string literal.
// You can concatenate strings using the + operator or the push_str() method.
// You can access individual characters in a string using indexing or the chars() method.
// You can iterate over the characters in a string using a for loop or the chars() method.
// You can convert a String to a &str using the as_str() method or by using a reference.
// You can convert a &str to a String using the to_string() method or the String::from() function.
fn main() {
    let mut my_string = String::from("Hello, ");
    my_string.push_str("world!");
    println!("{}", my_string);  

    let greeting = "Hello, Rust!";
    let mut farewell = greeting.to_string();
    farewell.push_str(" Goodbye, Rust!");
    println!("{}", farewell);

    let s1: String = "whenever".to_string();
    let first_char: char = s1.chars().nth(0).unwrap();
    println!("First character: {}", first_char);

    let s2: String = String::from("hello");
    println!("s2 = {}", s2);

    let s3: String = s1 + &s2; // s1 is moved here and cannot be used again
    println!("The value of s3 = {}", s3);

    // Mutate the variable
    let mut s: String = String::from("goodbye");
    s.push_str(", world!");
    s.push('!'); // push() appends a char to a String
    println!("The value of s = {}", s); // This will print "goodbye, world!!"


    // Formatting String
        let salam = "Salam";
    let salut = "Aleikunm";
    let full_message: String = format!("{salam} {salut}");
    println!("{full_message}");
}