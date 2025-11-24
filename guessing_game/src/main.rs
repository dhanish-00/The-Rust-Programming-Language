
use std::io;

use rand::Rng;

fn main() {
    println!("Guss the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is :{secret_number}");
        
    println!("please input your guss!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    println!("You gussed: {guess}");
    
}
 
