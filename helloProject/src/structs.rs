// Structs are custom data types that let you name and package together multiple related values that make up a meaningful group. They are similar to classes in other programming languages but do not have methods associated with them.
// Structs are defined using the struct keyword, followed by the struct name and a block containing the fields of the struct.
// Structs can have named fields, tuple-like fields, or be unit-like (without any fields).
// Structs are useful for creating complex data types that group related data together, making code more organized and easier to understand.
// Example of defining and using a struct:

// fn main() {
//     let rect: (i32, i32) = (30, 50);
//     println!("The area of the rectangle is {} square pixels.", area(rect));

//     // Struct
//     struct Book {
//         title: String, 
//         author: String,
//         pages: u32,
//         price: f64,
//         available: bool,
//     }

//     struct User{
//         active: bool,
//         username: String,
//         email: String,
//         sign_in_count: u64,
//     }

//     let mut user1: User = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someusername@gmail.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@gmail.com");
//     println!("User1 email: {}", user1.email);
// }



// Structs are custom data types that let you name and package together multiple related values that make up a meaningful group. They are similar to classes in other programming languages but do not have methods associated with them.
// Structs are defined using the struct keyword, followed by the struct name and a block containing the fields of the struct.
// Structs can have named fields, tuple-like fields, or be unit-like (without any fields).
// Structs are useful for creating complex data types that group related data together, making code more organized and easier to understand.
// Example of defining and using a struct:

struct Book {
    title: String, 
    author: String,
    pages: u32,
    price: f64,
    available: bool,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn area(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn main() {
    let rect: (i32, i32) = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area(rect));

    let mut user1: User = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someusername@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@gmail.com");
    println!("User1 email: {}", user1.email);

    // returning struct from function
    fn build_user(email: String, username: String) -> User {
        User {
            email, // field init shorthand
            username, // field init shorthand
            active: true,
            sign_in_count: 1,

        };
    }

    // Create instances from others instances
    let user2: User = User{
        email: String::from("another@gmail.com"),
        ..user1
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(255, 255, 255);

    // Unit-Liike Structs
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);
    println!("Subject: {:?}", std::any::type_name::<AlwaysEqual>());
}