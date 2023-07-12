
use std::env;
use parsing::parse_json::parse_json;

mod parsing;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

	if args.len() == 2 {
		parse_json("test")
	}
    else {
		println!(
			"usage: ft_turing [-h] jsonfile input
			positional arguments:
			jsonfile json description of the machine
			input input of the machine
			optional arguments:
			-h, --help show this help message and exit")
	}
}
