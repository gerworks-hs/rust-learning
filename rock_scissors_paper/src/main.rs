const ROCK: i8 = 1;
const PAPER: i8 = 2;
const SCISSORS: i8 = 3;

fn main() {
    print!("1->Rock\n2->Paper\n3->Scissors\n");

	loop {
		let program_option: i8 = match fastrand::i8(1..=3) {
			1 => ROCK,
			2 => PAPER,
			3 => SCISSORS,
			_ => -1
		};

		let mut user_option: String = std::string::String::new();

		std::io::stdin()
			.read_line(&mut user_option)
			.expect("Failed to read input");

		let user_option: i8 = match user_option.trim().parse() {
			Ok(option) => option,
			Err(_) => {
				println!("Invalid option");
				continue;
			}
		};

		match (program_option, user_option) {
			(ROCK, SCISSORS) | (SCISSORS, PAPER) | (PAPER, ROCK) => {
				println!("Program option: {}", program_option);
				println!("Your option {}", user_option);
				println!("You loose");
				break;
			}
			(SCISSORS, ROCK) | (ROCK, PAPER) | (PAPER, SCISSORS) => {
				println!("Program option: {}", program_option);
				println!("Your option {}", user_option);
				println!("You win");
				break;
			}
			_ => {
				println!("Program option: {}", program_option);
				println!("Your option {}", user_option);
				println!("Tie");
				continue;
			}
		}
	}
}
