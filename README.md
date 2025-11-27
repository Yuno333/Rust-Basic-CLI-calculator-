# Rust-Basic-CLI-calculator-
Rust simple cli calculator with explanation of code for beginners 


# Save as main.rs, then:
rustc main.rs
./main

# Or if using Cargo:
cargo run
```

**Example usage:**
```
Enter calculation: 10 + 5
Result: 15

Enter calculation: 20 / 4
Result: 5

Enter calculation: 7 * 3
Result: 21


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
