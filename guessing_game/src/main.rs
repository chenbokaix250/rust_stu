use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("guess a number!");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("unable to read ");

        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("your guess is:{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Equal=> {
                println!("You win");
                break;
            }
            Ordering::Greater=>println!("Too big"),
        }
    }
}
