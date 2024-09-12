fn main() {
	println!("Welcome, guess a number between 1 and 100");
	let rand_number: u8 = fastrand::u8(1..=100);
	
	loop {
		let mut user_guess: String = std::string::String::new();

		std::io::stdin()
			.read_line(&mut user_guess)
			.expect("Failed to read line");
		
		let user_guess: u8 = match user_guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please enter a valid number");
				continue;
			}
		};

		if rand_number > user_guess {
			println!("More");
		} else if rand_number < user_guess {
			println!("Less");
		} else {
			println!("Match");
			break;
		}
	}
}