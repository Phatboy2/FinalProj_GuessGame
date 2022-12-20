extern crate rand;
extern crate colored;

use std::io;
use rand::Rng;
use colored::*;

static GREETING: &str = "High or Low!";
static FAREWELL: &str = "Thank you for playing!";
static WIN: &str = "Congratulations! You won!";
static LOSE: &str = "Sorry! You lose!";
static HIGHER: &str= "Higher";
static LOWER: &str= "Lower";

///Code is designed to show the user one card and ask if the next card will be higher or lower than the first, then congratulatecls or forgive the user
fn main() {
    println!("{} \n", GREETING.yellow().bold().italic());

    let first_number = rand::thread_rng().gen_range(1..101); // .gen_range is from Rng.
    let secret_number = rand::thread_rng().gen_range(1..101); // .gen_range is from Rng.

    intro();

    println!("First Number is: {}", first_number);
    println!("Second Card is Hidden");

    println!("Is the second card {} or {} than the first? ", HIGHER.cyan().italic(), LOWER.cyan().italic());

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();

    println!("Second card was: {}", secret_number);

    compare(first_number, secret_number, guess);

    println!("{}", FAREWELL.yellow().bold().italic());

}

fn intro() {
    println!("Welcome to my Rust Final Project! by Ben Gelfand");
    println!("The game is simple: the program will generate two random numbers. One will be revealed and one will be kept secret.");
    println!("The program will then ask the user if they think the secret card is higher or lower than the first card");
    println!("Depending on what the user answers and the otucome, the program will display you win or you lose. \n");
}

fn compare(first_number:i32, second_number:i32, guess: String) {

    if second_number > first_number {
        println!("Second card was {}", HIGHER.cyan().bold());

        if guess.contains("High") || guess.contains("high") {
            println!("{}", WIN.green().bold());
        } else  if guess.contains("Low") || guess.contains("low") {
            println!("{}", LOSE.red().bold());
        }
    } else  if second_number < first_number {
        println!("Second card was {}", LOWER.cyan().bold());

        if guess.contains("High") || guess.contains("high") {
            println!("{}", LOSE.red().bold());
        } else  if guess.contains("Low") || guess.contains("low") {
            println!("{}", WIN.green().bold());
        }
    }

}

