use std::io;//Input and output use
use rand::Rng; //Random number library
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new(); //New immutable variable

        io::stdin()
            .read_line(&mut guess)//append user input to variable
            .expect("Failed to read line");//error handling

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
