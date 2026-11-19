// Exercise 1
// use std::fs;

// fn read_file_contents(path:&str)->Result<String, String>{
//     fs::read_to_string(path).map_err(|e| format!("FAiled to read file '{}':{}",path,e))
// }

// fn main(){
//     let exists=read_file_contents("closure1.rs");
//     let missing=read_file_contents("nonexistent.txt");

//     match exists{
//         Ok(content) => println!("File contents:\n{}", content),
//         Err(error) => println!("Error: {}", error),
//     }

//     match missing {
//         Ok(content) => println!("File contents:\n{}", content),
//         Err(error) => println!("Error: {}", error),
//     }
// }


//Exercise 2
// fn parse_even_number(s: &str) -> Result<u32, String> {
//     let number: u32 = match s.parse() {
//         Ok(n) => n,
//         Err(_) => return Err(String::from("Invalid number format")),
//     };

//     if number == 0 {
//         return Err(String::from("Number must be positive"));
//     }

//     if number % 2 != 0 {
//         return Err(String::from("Number must be even"));
//     }

//     Ok(number)
// }

// fn main() {
//     assert_eq!(parse_even_number("42"), Ok(42));
//     assert_eq!(
//         parse_even_number("43"),
//         Err(String::from("Number must be even"))
//     );
//     assert_eq!(
//         parse_even_number("abc"),
//         Err(String::from("Invalid number format"))
//     );
//     assert_eq!(parse_even_number("-42"), Err(String::from("Invalid number format")));


//     println!("All tests passed!");
// }

//Exersice 3
// struct User {
//     username: String,
//     age: u8,
// }

// fn validate_user(username: String, age: u8) -> Result<User, Vec<String>> {
//     let mut errors = Vec::new();

//     if username.len() < 3 || username.len() > 20 {
//         errors.push(String::from(
//             "Username must be between 3 and 20 characters"
//         ));
//     }

//     if !username.chars().all(|c| c.is_ascii_alphanumeric()) {
//         errors.push(String::from(
//             "Username must contain only alphanumeric characters"
//         ));
//     }

//     if age < 13 || age > 120 {
//         errors.push(String::from(
//             "Age must be between 13 and 120"
//         ));
//     }

//     if !errors.is_empty() {
//         return Err(errors);
//     }

//     Ok(User { username, age })
// }

// fn main() {
//     let valid = validate_user("johnDoe".to_string(), 25);

//     match valid {
//         Ok(user) => println!("Valid user: {}, age {}", user.username, user.age),
//         Err(errors) => println!("Validation errors: {:?}", errors),
//     }

//     let invalid = validate_user("x".to_string(), 10);

//     match invalid {
//         Ok(user) => println!("Valid user: {}", user.username),
//         Err(errors) => {
//             for error in errors {
//                 println!("Error: {}", error);
//             }
//         }
//     }
// }


// Exercise 4
// fn sum_results(
//     a: Result<i32, String>,
//     b: Result<i32, String>,
// ) -> Result<i32, String> {
//     match (a, b) {
//         (Ok(x), Ok(y)) => Ok(x + y),
//         (Err(e), _) => Err(e),
//         (_, Err(e)) => Err(e),
//     }
// }

// fn main() {
//     assert_eq!(sum_results(Ok(1), Ok(2)), Ok(3));

//     assert_eq!(
//         sum_results(Ok(1), Err("error".to_string())),
//         Err("error".to_string())
//     );

//     assert_eq!(
//         sum_results(Err("first error".to_string()), Ok(2)),
//         Err("first error".to_string())
//     );

//     assert_eq!(
//         sum_results(
//             Err("first error".to_string()),
//             Err("second error".to_string())
//         ),
//         Err("first error".to_string())
//     );

//     println!("All tests passed!");
// }

// Exercise 5
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum MathError {
    ParseError(ParseIntError),
    DivisionByZero,
}

fn divide_strings(a: &str, b: &str) -> Result<f64, MathError> {
    let num_a: i32 = a
        .parse()
        .map_err(MathError::ParseError)?;

    let num_b: i32 = b
        .parse()
        .map_err(MathError::ParseError)?;

    if num_b == 0 {
        return Err(MathError::DivisionByZero);
    }

    Ok(num_a as f64 / num_b as f64)
}

fn main() {
    println!("{:?}", divide_strings("4", "2"));
    println!("{:?}", divide_strings("a", "2"));
    println!("{:?}", divide_strings("4", "0"));
}

