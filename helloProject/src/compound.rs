// Compound Data Types [Arrays, Tuples, Slice, Strings, (Slice String) Structs]

// Arrays is a collection of values of the same type
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers Array: {:?}", numbers);


    // let mix = [1,2, "apple", true];
    // println!("Mix Array: {:?}", mix);

    let fruits: [&str; 4] = ["apple", "banana", "cherry", "mango"];
    println!("Fruits Array 1st  element: {:?}", fruits [0]);
    println!("Fruits Array 2nd  element: {:?}", fruits [1]);
    println!("Fruits Array 3rd  element: {:?}", fruits [2]);
    println!("Fruits Array 4th  element: {:?}", fruits [3]);


    // Tuple is a collection of values of different types
    

      // Define a tuple
      let human: (String, i32, bool) = ("Alice".to_string(), 30, true);

      // Print the whole tuple
      println!("Human Tuple: {:?}", human);
  
      // Access elements
      println!("Name: {}", human.0);
      println!("Age: {}", human.1);
      println!("Active: {}", human.2);

      let my_mix_tuple = ("Kratos", 20, true, ["apple", "banana", "cherry", "mango"]);
      println!("My Mix Tuple: {:?}", my_mix_tuple);


      // Slice
         let number_slice: &[i32] = &[1,2,3,4,5];
         println!("Number Slice: {:?}", number_slice);

         let animal_slice: &[&str] = &["dog", "cat", "bird", "fish"];
         println!("Animal Slice: {:?}", animal_slice);

         let book_slice: &[&String] = &[&"IT".to_string(), &"The Great Gatsby".to_string(), &"The Catcher in the Rye".to_string()];
         println!("Book Slice: {:?}", book_slice);


         // String vs String Slice (&str)
         // String [ growable, mutable, owned string  type]
         // all variable in rust language are imutable by default
         let mut stone_code = String::from("Stone Code");
         
         stone_code.push_str(" is a programming language");

         println!("Stone Code: {}", stone_code);

         // B- &str [String Slice]
         let string: String = String::from("Hello, Worlds of Rust");
         let slice: &str = &string[0..5];
         println!("Slice value: {}", slice);


         
         
         


         

            // Two types of Formatting:
            // 1. Display Formatting: {}
            // 2. Debug Formatting: {:?}



            let name = "Alice";
            let age = 30;
            println!("Name: {}, Age: {}", name, age);

            println!("Name: {:?}, Age: {:?}", name, age);
}



