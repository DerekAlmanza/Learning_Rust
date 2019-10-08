use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number.");
    let secret_number = rand::thread_rng().gen_range(0, 100);
    println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        println!("Please input your guess: {}", guess);

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(if_is_the_good_type_of_data) => if_is_the_good_type_of_data,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small\n"),
            Ordering::Greater => println!("Too big\n"),
            Ordering::Equal => {
                println!("Is equal, you win!");
                break;
            }
        };
    }
}


