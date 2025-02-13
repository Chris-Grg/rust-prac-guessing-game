use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Guess a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_)=> continue,
        };
        println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Higher than that"),
        Ordering::Greater => println!("Lower than that"),
        Ordering::Equal => {
            println!("You guessed correctly!");
            break;
        }  
    }
    }



}
