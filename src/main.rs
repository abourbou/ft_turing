
use std::env;
use parsing::parse_json::parse_json;
use process_turing::process::process;

mod parsing;
mod process_turing;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

	if args.iter().any(|arg| arg == "--help" || arg == "-h"){
		println!(
				"usage: ft_turing [-h] jsonfile input
				positional arguments:
				jsonfile json description of the machine
				input input of the machine
				optional arguments:
				-h, --help show this help message and exit");
			return;
	}

	if args.len() != 2 {
		println!("Wrong number of arguments, usage: ft_turing [-h] jsonfile input");
		return;
	}
	let desc = parse_json(&args[0]);

	process(desc, &args[1]);

}
