use std::io::Write;

#[derive(Debug)]
enum GameValue {
	Rock,
	Paper,
	Scissors
}

fn main() {
    print!("1->Rock\n2->Paper\n3->Scissors\n");

	loop {
		let program_option: GameValue = match fastrand::i8(1..=3) {
			1 => GameValue::Rock,
			2 => GameValue::Paper,
			3 => GameValue::Scissors,
			_ => unreachable!()
		};

		let mut user_option: String = std::string::String::new();

		print!("-> ");
		std::io::stdout().flush().expect("Failed to reach standard output");

		std::io::stdin()
			.read_line(&mut user_option)
			.expect("Failed to reach standard input");

		let user_option: GameValue = match user_option.trim().parse() {
			Ok(option) => match option {
				1 => GameValue::Rock,
				2 => GameValue::Paper,
				3 => GameValue::Scissors,
				_ => {
					println!("Invalid option");
					continue;
				}
			}
				
			Err(_) => {
				println!("Invalid option");
				continue;
			}
		};

		match (&program_option, &user_option) {
			(GameValue::Rock, GameValue::Scissors) | (GameValue::Scissors, GameValue::Paper) | (GameValue::Paper, GameValue::Rock) => {
				println!("Program option: {:?}", program_option);
				println!("Your option: {:?}", user_option);
				println!("You loose");
				break;
			}
			(GameValue::Scissors, GameValue::Rock) | (GameValue::Rock, GameValue::Paper) | (GameValue::Paper, GameValue::Scissors) => {
				println!("Program option: {:?}", program_option);
				println!("Your option: {:?}", user_option);
				println!("You win");
				break;
			}
			_ => {
				println!("Program option: {:?}", program_option);
				println!("Your option: {:?}", user_option);
				println!("Tie");
				continue;
			}
		}
	}
}