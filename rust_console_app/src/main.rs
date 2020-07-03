use rand::Rng;
use std::io;

fn main() {
    println!("==+==+==+==+==+==+==+==+==+==+==+===");
    println!("Hello, welcome to rust consol app");
    println!("This app will generate a random number between 1 and 100, and you will guess what the number is");
    println!("Everytime you guess a number, the computer will tell you if the number is larger, smaller or equal to the number");
    println!("Let get started!");

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0, 100);
    let mut input_number = -1;
    let mut tries = 1;

    while input_number != random_number {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input_number = input.trim().parse().unwrap_or(0);
            }
            Err(e) => println!("Something went wrong: {}", e),
        }

        if input_number < random_number {
            println!(
                "The number you entered {} is smaller than the number, please try again: ",
                input_number
            );
            tries += 1;
        } else if input_number > random_number {
            println!(
                "The number you entered {} is larger than the number, please try again: ",
                input_number
            );
            tries += 1;
        } else {
            println!("Congratulations! The number is {}.", random_number);
            println!("You get the number in {} tries.", tries);
        }
    }
}
