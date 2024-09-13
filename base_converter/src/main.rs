fn main() {
	let argv: Vec<String> = std::env::args().collect::<Vec<String>>();
	let argc: usize = argv.len();

	if argc < 3 || argc > 3 {
		println!("Usage: {:?} <from base> <to base>", &argv[0]);
		std::process::exit(1);
	} else {
		println!("From base {} to base {}", &argv[1], &argv[2]);
	}

	let from_base: u8 = argv[1].parse::<u8>().expect("Cannot convert first argument");
	let to_base: u8 = argv[2].parse::<u8>().expect("Cannot convert second argument");

	match from_base {
		2 => {()},
		10 => {()},
		_ => {
			panic!("Invalid first base, possible bases are 2 and 10");
		}
	}
	match to_base {
		2 => {()},
		10 => {()},
		_ => {
			panic!("Invalid second base, possible bases are 2 and 10");
		}
	}

}