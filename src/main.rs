use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let mut count = 0u32;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Welcome to the guessing game! You have 10 tries to guess the secret number. If you don't guess it in 5 tries, I'll give you a hint. 
    If you don't guess it in 10 tries, I'll tell you the secret number. 
    Good luck!");
    
    loop {
    count += 1;
    println!("Please input your guess or type exit to exit.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if guess.trim() == "exit" {
        println!("Goodbye!");
        break;
    }

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That's not a number silly!");
            continue
        }
    };




    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            println!("The secret number is: {}", secret_number);
            break;

        }
    }
    if count == 5 {
        if secret_number % 2 == 0 {
            println!("The secret number is even!");
        } else {
            println!("The secret number is odd!");
        }
    }

    if count == 6 {
        if secret_number > 50 {
            println!("The secret above 50");
        } else {
            println!("The secret number is below 50!");
        }
    }

    if count == 8 {
        if secret_number > 70 {
            println!("The secret above 70");
        } else if secret_number > 90 {
            println!("The secret above 90");
        } else if secret_number < 20 {
            println!("The secret below 20");
        } else {
            println!("The secret number is between 20 and 70");
        }

    }
    
    if count == 10 {
        println!("You lose!");
        println!("The secret number is: {}", secret_number);
        break;
    }


    println!("You guessed: {}", guess);

    }

}
