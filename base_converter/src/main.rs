fn main() {

	let argv: Vec<String> = std::env::args().collect::<Vec<String>>();
	let argc: usize = argv.len();

	if argc < 4 || argc > 4 {
		println!("Usage: {:?} <from base> <to base> <number>", &argv[0]);
		std::process::exit(1);
	} else {
		println!("Number <{}> from base <{}> to base <{}>", &argv[3], &argv[1], &argv[2]);
	}

	let from_base: u8 = argv[1].parse::<u8>().expect("Cannot convert first argument");
	let to_base: u8 = argv[2].parse::<u8>().expect("Cannot convert second argument");

	match (from_base, to_base) {
		(2, 10) => {
			match usize::from_str_radix(&argv[3], 2) {
				Ok(value) => {
					println!("Binary <{}> is <{}> decimal", &argv[3], value);
				}
				Err(_) => {
					println!("Cannot convert number");
					std::process::exit(1);
				}
			}
		}
		(10, 2) => {
			let number: usize = argv[3].trim().parse::<usize>().expect("Cannot convert number");
			
			let binary: String = format!("{:b}", number);

			println!("Decimal <{}> equals to <{}> binary", number, binary);
		}
		_ => {
			println!("Invalid bases or they are the same");
			std::process::exit(1);
		}
	}

}