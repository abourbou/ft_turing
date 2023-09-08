
use crate::parsing::parse_json::{MachineDescription, Transition};
use colored::Colorize;
use tailcall::tailcall;

pub fn print_state(tape: &str, state: &str, next_tran: &Transition, index: i32) {
	print!("[");
	tape.chars().take(index as usize).for_each(|c| print!("{}", c));
	print!("{}", tape.chars().nth(index as usize).unwrap().to_string().color("red").bold().on_bright_black());
	tape.chars().skip((index + 1) as usize).for_each(|c| print!("{}", c));
	print!("]");

	let side : &str = if next_tran.go_right{"RIGHT"} else {"LEFT"};
	println!(" ({}, {}) -> ({}, {}, {})", state, next_tran.read, next_tran.to_state, next_tran.write, side);
}

#[tailcall]
pub fn turing_process_rec(tape: String, state: &str, desc: &MachineDescription, index: i32)
{
	let find_final = desc.finals.iter().find(|&final_state| state == final_state);
	if let Some(final_state) = find_final {
		println!("[{}] {}", tape.color("green").bold(), final_state);
		return;
	}

	let read: char = tape.chars().nth(index as usize).unwrap();
	let tran: Option<&Transition> = desc.list_transitions.get(state).unwrap()
							.iter().find(|t: &&Transition| t.read == read);

	if let Some(transition) = tran {
		print_state(&tape, state, transition, index);
		let step = if transition.go_right { 1 } else { -1 };
		let next_index = index + step;
		let new_tape = build_tape(tape, index as usize, transition.write, next_index, desc);

		if next_index == -1 {
			turing_process_rec(new_tape, &transition.to_state, desc, 0)
		}
		else {
			turing_process_rec(new_tape, &transition.to_state, desc, next_index)
		}
	}
	println!("No transition for {} with {}", state, read);
}

pub fn build_tape(tape: String, index_to_change: usize, char_to_replace: char, next_index: i32, desc: &MachineDescription) -> String {
	let modified_string: String = tape
		.chars()
		.enumerate()
		.map(|(i, c)| if i == index_to_change { char_to_replace } else { c })
		.collect();

	if next_index < 0 {
		return desc.blank.to_string() + &modified_string;
	}
	else if next_index as usize >= modified_string.len() {
		return modified_string + &desc.blank.to_string();
	}

	modified_string
}

pub fn process(desc: MachineDescription, tape: &str) {
	turing_process_rec(tape.to_string(), &desc.initial, &desc, 0);
}
