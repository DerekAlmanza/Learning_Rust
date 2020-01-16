fn main() {

    // We define a variable and we use the structure.
    let mut user1 = User {
        email: String::from("jose.adolfo@gmail.com"),
        username: String::from("Jose64"),
        sign_in_count: 3,
        active: true,
    };

    // If we wish only define a one attribute we put this.
    user1.username = String::from("Loquendo97");

    println!("{}", user1.username);
}

// To define a struct, we put in the keyboard `struct` and the name of structure.
// Inside of the curly brackets, we define the names and the data types of the pieces of data.
// Let's call this as "fields".
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

