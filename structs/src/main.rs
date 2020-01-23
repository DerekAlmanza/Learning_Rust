fn main() {

    // We define a variable and we use the structure.
    // We must define the variable like mutable for can modifies it.
    let mut user1 = User {
        email: String::from("jose.adolfo@gmail.com"),
        username: String::from("Jose64"),
        sign_in_count: 3,
        active: true,
    };

    // If we wish only define a one attribute we put this.
    user1.username = String::from("Loquendo97");
    println!("{}", user1.username);

    // But if we wish modifies some attributes but keep values of other variable we can do this:
    let user2 = User {
        email: String::from("anothername@gmail.com"),
        username: String::from("another name"),
        //sign_in_count: user1.sign_in_count,
        //active: user1.active,
        ..user1
    };

    // Tuple Structs.
    // It's a simpler way for build structs, without a name variables, only the data type
    // and how many attributes you wish in struct.
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("{}", black.0); // To access a individual value we use the point notation.

    let rectangle_1 = RectangleDimensions{
        width: 50,
        height: 30,
    };

    println!("The area of rectangle is {}\n", area(&rectangle_1));

    // If we wish print the data struct we must put this:
    // The `:#?` operator serves to print the structure as is.
    println!("rect1 is {:#?}", rectangle_1);


}


// We can build a function that returns a `User` instance with the given username and email.
// As the variable names as the same, we can put only the variable name without the two points.
// An example of this is the username variable and the email variable.
fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        sign_in_count: 2,
        active: true,
    }
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


// We will make an example of structs and we will calculate the rectangle area.
// We have to explicitly choose to make the functionality `debug` available for our structure.
#[derive(Debug)]
struct RectangleDimensions {
    width: u32,
    height: u32,
}


// Function that calculates the rectangle area.
fn area(rectangle: &RectangleDimensions) -> u32 {
    let res = rectangle.width * rectangle.height;
    res
}
