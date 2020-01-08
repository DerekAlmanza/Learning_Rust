fn main() {
    
    variables_and_mutability();
    constants();
    shadowing();
    data_types();
    functions();
    control_flow();

}

// *VARIABLES AND MUTABILITY*.
fn variables_and_mutability() {

    // If we put let x = 5; the code will fail because later the x value will change.
    let mut x= 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("Now, the value of x is {}\n", x);

}

// *CONSTANTS.*
fn constants() {
 
    // Rust’s naming convention for constants is to use all
    // uppercase with underscores between words, and underscores can be
    // inserted in numeric literals to improve readability.
    const MAX_POINTS: u32 = 100000;

}

// SHADOWING.
fn shadowing() {
 
    // Use the same variable name repeatdly for different uses.
    let y:u32 = 6; // Here we specify the data type because otherwise it wouldn't compile the line 20. 
    println!("The value of y is {}", y);
    let y = y + 2;
    println!("The value of y is {}", y);
    let y = y.pow(2);
    println!("The value of y is {}\n", y);

}

// DATA TYPES.
fn data_types() {
    
    // ** Integer types.
    // If the number is signed you must put i{number bytes}, otherwise you put u{number bytes}.
    let x: u32 = 128; //Examples. We have i/u8, i/u16, i/u32, i/u64, i/u128.
    let y: i32 = -128; 

    // ** Float types.
    // There are two data types for this type of case. f64 by default and f32 if 
    // we specify in the variable.
    let float_default = 2.0; // f64. More precise than f32.
    let float_not_default: f32 = 2.0; // f32

    // ** Numeric operations.
    // Addition.
    let sum = 5 + 10;
    println!("The sum is {}", sum);

    // Substraction
    let substraction = 82.2 - 90.3;
    println!("The substraction is {}", substraction);

    // Division
    let division = 56.7 / 32.2;
    println!("The division is {}", division);

    //Multiplication
    let multiplication = 4 * 30;
    println!("The multiplication is {}", multiplication);

    // Remainder - residuo
    let remainder = 40 % 8;
    println!("The remainder is {}\n", remainder);

    // An if expression allows you branch your code depending on conditions.
    // "If this condition is met, run this block of code." 
    // "If this condition is not met not run this block of code."// ** Boolean types.
    let t: bool = true;
    let f: bool = false;
    println!("The sky is purple? {}\nThe red color is red? {}\n", f, t);

    // ** Char type.
    let letter_z = 'z';
    let black_cat = '😻';
    println!("{} and {}\n", letter_z, black_cat);

}

// *FUNCTIONS*
fn functions() {
    
    println!("This is a first function.");
    another_function();
    function_with_parameters(5, 2.03);
    println!("The value of the function with return values is: {}", function_with_return_values());

}

fn another_function() {
    
    println!("This is a function that is called for a second different function.\n");

}

fn function_with_parameters(x: i32, y: f64) {

    println!("The value of x is: {}", x);
    println!("The value of y is: {}\n", y);

}

//Declarations: Instructions that not return nothing.
//Expressions: Evaluate to a resulting value.
fn function_with_return_values() -> i32 {
    5
}

// CONTROL FLOW
fn control_flow() {

    if_control_flow();
    loop_control_flow();
    while_control_flow();
    for_control_flow();

}

// *If control flow*
// An if expression allows you branch your code depending on conditions.
// "If this condition is met, run this block of code." 
// "If this condition is not met not run this block of code."
fn if_control_flow() {

    let number = 3;

    if number == 3 {
        println!("\nThe number is equal to 3\n");
    } else if number < 5 {
        println!("The  number is smaller than 5\n");
    } else {
        println!("The number is bigger than 5\n");
    } 

    // Using `if` in a `let` statement
    let condition = true;
    let number = if condition {
                     5
                 } else {
                     6
                 };

    println!("The value of number is {}\n", number);
    
}

// *loop control flow*
// The loop keyword tells Rust to execute a block of code 
// over and over again forever until you explicitly tell it to stop.
fn loop_control_flow() {

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is {}\n", result);

}

// *While control flow*
// It’s often useful for a program to evaluate a condition within a loop. 
// While the condition is true, the loop runs.
fn while_control_flow() {

    let mut number = 3;

    while number != 0 {
        println!("{}",number);
        number -= 1;
    }
    println!("Stop right there madafaka\n");
}

// *For control flow*
// `For` it's useful to loop over the elements of a collection. 
fn for_control_flow() {

    let array = [10,20,30,40,50];

    for i in array.iter() {
        println!("The value is: {}", i);
    }
}