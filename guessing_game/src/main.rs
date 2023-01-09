use std::cmp::Ordering;
use std::io;
use colored::*;
use rand::Rng;

fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!(" The secret number is : {} ", secret_number);
    

    loop {
        println!("Please input your number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_) => continue,
        };

        println!("you have guessed: {} ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small !".red()),
            Ordering::Greater => println!("{}","Too big !".red()),
            Ordering::Equal => {
                println!("{}","you win !".green());
                break;
            }
        }
    }
}
