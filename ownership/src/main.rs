fn main() {
    // Ownership.

    //Variable scope.  // `random_variable` is not valid here, it’s not yet declared
    let random_variable = String::from("Hello world"); // random_variable is not valid here, it’s not yet declared

    let mut s1 = String::from("hello");
    s1.push_str(", world");
    println!("{}\n", s1);

    // Cloning strings can be expensive. 
    let s2 = s1.clone();
    println!("s1 = {}\ns2 = {}", s1, s2);

    let gives_ownership_from_function = gives_ownership();
    let normal_string = String::from("hello");
    let takes_the_ownership_value = takes_and_gives_back
                                    (&gives_ownership_from_function);
    
    println!("{}", gives_ownership_from_function);
    println!("{}", normal_string);
    println!("{}", takes_the_ownership_value);

    string_slices();



} // The random_variable scope is now over.


// The returning values can also transfer the ownership.
fn gives_ownership() -> String {

    let owner = String::from("hello");
    owner

}

// & is defined as reference, we use this for take a value without 
// possess it. And &mut in case of you want that your reference to be mutable.
fn takes_and_gives_back(some_string: &String) -> &String { // some_string is a reference to String.

    some_string

}

// *String slices*
fn string_slices() {

    let hello_world_string = String::from("hello world");
    let len = hello_world_string.len();
    let hello = &hello_world_string[0..len]; // The references borrows their ownership for the first byte to end of the string.
    let hello2 = &hello_world_string[..]; //This is equal as the line above.
    println!("{}", hello);

}
