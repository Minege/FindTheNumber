extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut secret_number = 0;

    //println!("The secret number is: {}", secret_number);
	loop{
        let mut cur_game = true;
        let mut mistakes = 0;
		println!("Enter \"exit\" to leave the game.");
		println!("Welcome to Guess The Number !\n1. Range 0 to 50\n2. Range 0 to 100\n3. Range 0 to 150\n4. Range 0 to 200\n5. Custom Range :D");
        println!("Please input your choice : ");

		let mut choice = String::new();
        if choice.trim() == "exit"{
            println!("Exiting");
            return;
        }
		io::stdin().read_line(&mut choice)
				.expect("failed to read line");
		let choice: u8 = match choice.trim().parse(){
			Ok(int) => int,
			Err(_) => {println!("Please enter a correct number !");continue}
		};

		match choice{
			1 => {secret_number = rand::thread_rng().gen_range(1, 51);},
            2 => {secret_number = rand::thread_rng().gen_range(1, 101);},
            3 => {secret_number = rand::thread_rng().gen_range(1, 151);},
            4 => {secret_number = rand::thread_rng().gen_range(1, 201);},
            5 => {let range = custom_range();secret_number = rand::thread_rng().gen_range(range.0, range.1);}
			_ => {continue;}
		}

        println!("Now select when to stop the game because of mistakes");
        println!("Number of mistakes : ");
        let mut max_mistakes = String::new();
		io::stdin().read_line(&mut max_mistakes)
				.expect("failed to read line");
		let max_mistakes: u16 = match max_mistakes.trim().parse(){
			Ok(int) => int,
			Err(_) => {println!("Because you didn't entered anything valid, the limit of mistakes will be 5");5}
		};
        println!("!!!! THE GAME BEGIN !!!!");
		while cur_game {

			println!("Please input your guess : ");

			let mut guess = String::new();

			io::stdin().read_line(&mut guess)
				.expect("failed to read line");

			if guess.trim() == "exit"{
				println!("Exiting");
				return;
			}

			//let guess: u32 = guess.trim().parse();
			let guess: u32 = match guess.trim().parse(){
				Ok(int) => int,
				Err(_) => {println!("Please enter a number !");continue}
			};


			match guess.cmp(&secret_number) {
				Ordering::Less    => {println!("Too small!\n");mistakes += 1},
				Ordering::Greater => {println!("Too big!\n");mistakes += 1},
				Ordering::Equal   => {println!("You win with {} mistakes !\n\n\n\n\n", mistakes);cur_game=false}
			}


            if mistakes >= max_mistakes{
                println!("Loose. You tryed too many time ! ( {} mistakes ). Maybe do you want to retry ?\nThe correct number was {} ! Better luck the next time !\n\n\n\n\n", mistakes, secret_number);

                cur_game = false
            }
		}
	}
}

fn custom_range() -> (u32, u32){
    println!("You choosed a special range ! Please complete : ");
    println!("From : ");
    let mut min = String::new();
    let mut max = String::new();
    loop {
        io::stdin().read_line(&mut min)
            .expect("failed to read line");
        let min: u32 = match min.trim().parse() {
            Ok(int) => int,
            Err(_) => {
                println!("Please enter a correct number !");continue
            }
        };
        println!("To : ");
        io::stdin().read_line(&mut max)
            .expect("failed to read line");
        let max: u32 = match max.trim().parse() {
            Ok(int) => int,
            Err(_) => {
                println!("Please enter a correct number !");continue
            }
        };
        return (min, max);
    }

}
