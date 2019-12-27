fn main() {
    
    // Variables and mutability.
    // If we put let x = 5; the code will fail because later the x value will change.
    let mut x= 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("Now, the value of x is {}\n", x);

    // Constants. Rust’s naming convention for constants is to use all
    // uppercase with underscores between words, and underscores can be
    // inserted in numeric literals to improve readability.
    const MAX_POINTS: u32 = 100_000;

    // Shadowing. Use the same variable name repeatdly for different uses.
    let y:i32 = 6; // Here we specify the data type because otherwise it wouldn't compile the line 20. 
    println!("The value of y is {}", y);
    let y = y + 2;
    println!("The value of y is {}", y);
    let y = y.pow(2);
    println!("The value of y is {}", y);
    
}
