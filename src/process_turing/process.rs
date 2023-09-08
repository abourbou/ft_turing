

use crate::parsing::parse_json::{MachineDescription, Transition};

pub fn turing_process_rec(tape: String, state: String, desc: MachineDescription, index: i32)
{
	if desc.finals.iter().eq([state.clone()].iter()) {
		return;
	}
	println!("[{}] Etat {:?}", tape, state);
	
	
	let read: char = tape.chars().nth(index as usize).unwrap();
	let tran: Option<&Transition> = desc.list_transitions.get(&state).unwrap()
							.iter().find(|t: &&Transition| t.read == read);

	match tran {
		Some(transition) => {
			
			let step = if transition.go_right { 1 } else { -1 };
			let next_index = index as i32 + step;
			let new_tape = build_tape(tape.clone(), index as usize, transition.write, next_index.clone());

			if next_index == -1 {
				turing_process_rec(new_tape, transition.to_state.clone(), desc, 0)
			}
			else {
				turing_process_rec(new_tape, transition.to_state.clone(), desc, next_index)
			}
		}
		None => return ()
	}
}

pub fn build_tape(tape: String, index_to_change: usize, char_to_replace: char, next_index: i32) -> String {
	let modified_string: String = tape
		.chars()
		.enumerate()
		.map(|(i, c)| if i == index_to_change { char_to_replace } else { c })
		.collect();

	if next_index < 0 {
		return " ".to_string() + &modified_string;
	}
	else if next_index as usize > modified_string.len() { 
		return modified_string + &" ".to_string();
	}
		
	modified_string
}

pub fn process(desc: MachineDescription, tape: &str) {
	
	println!("{}", desc.initial);
	println!("{}", tape);


	turing_process_rec(tape.to_string(), desc.initial.clone() , desc, 0);
}