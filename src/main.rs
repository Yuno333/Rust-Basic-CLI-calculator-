//Rust Basic CLI calculator 



// we import necessary I/O utilities from the standard libary 
use std::io;

fn main() {
    println!("=== Rust CLL Calculator ===");
    println!("Enter calculations like: 5 + 3");
    println!("Supported Operations: +, -, *, /");
    println!("Type 'quit' to exit\n");


    loop {
        // Create a new string to store user input 
        let mut input  = String::new();

        print!("Enter calculations:  ");
        // flush stdout to enusre prompt appears before input 
        io::Write::flush(&mut io::stdout()).unwrap();

        //  read  user input from stdin 
        // read_line return a result that we handle with expect 
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //Remove whitespace from start and end 
        let input = input.trim();

        // check if user wants to quit 
        if input.eq_ignore_ascii_case("quit"){
            println!("Goodbye");
            break;
        }

        // Parse and calculate the result 
        match calculate(input) {
            Ok(result) => println!("Result: {}\n", result),
            Err(e) => println!("Error: {}\n", e), 
        }

    }
}

// function that parses input and perfoms calculations 
// Returns Result type - either Ok(f64) or Err(string)

fn calculate(input: &str) -> Result<f64, String> {
    // Split input into parts (numbers, operators, number)
    let parts: Vec<&str> = input.split_whitespace().collect();

    //check if we have exactly 3 parts 

    if parts.len() != 3 {
        return Err("Invalid format. Use: number operator number".to_string());
    }

    // Parse first number 
    //parse() returns Result<f64, ParseFloatError> 
     let num1: f64 = parts[0]
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", parts[0]))?;

      // Get operator 
    let operator = parts[1];

    // Parse second number
    let num2: f64 = parts[2]
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", parts[2]))?;

  

    // Perform calculation based on operator
    // match is rust's pattern matching - like switch/case in other languages

    match  operator{
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2), 
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("'{}' is not a supported operator", operator)),
    }
}

// ========= KEY RUST CONCEPTS EXPLAINED ============= //

/* 
1. OWNERSHIP & BORROWING
   - Each value in Rust has a single owner.
   - When the owner goes out of scope, the value is dropped.
   - References allow borrowing values without taking ownership.
   - Mutable references allow modifying borrowed values.
  
   - '&str' is a borrowed string slice (read only reference)
   - 'String' is an owned, growable string type.
    - '&mut' allows mutable borrowing.

2. RESULT TYPE: 
 - Result<T, E> represents either success (Ok(T)) or failure (Err(E)).
    - Used for error handling without exceptions.
    - The '?' operator propagates errors up the call stack.

3. ERROR HANDLING:
 - Rust encourages handling errors explicitly using Result and Option types.
 - 'map_err()' transforms errors from one type to another.
 - Pattern matching with 'match' allows handling different cases cleanly.
 - 'expect()' is used to handle unrecoverable errors by panicking with a message.

4. PATTERN MATCHING:
 - 'match' checks a value against patterns and executes corresponding code.
 - must handle all possible cases 
  - '-' is catch-all pattern

6. TYPE INFERENCE 
- rust infers types when possible 
- can explicitly annotate: 'let num: f64'

7. VECTORS
 - 'Vec<T>' is a growable array type.
 - 'collect()' gathers iterator items into a collection like Vec.

8. IMMUTABILITY BY DEFAULT
 - Variables are immutable unless declared with 'mut'.
 - Encourages safer code

*/