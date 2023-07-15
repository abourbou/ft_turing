
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct JsonMachineDescription {
    name: String,
    alphabet: Vec<String>,
    blank: String,
	states: Vec<String>,
	initial: String,
	finals: Vec<String>,
	transitions: HashMap<String, Value>,
}

pub fn parse_json(path : &str) {

	let error_path = "Couldn't find or load \"".to_owned() + &path + "\" json";
	let json_str = fs::read_to_string(path)
						.expect(&error_path);
	// println!("Open {}", path);

	let json_struct: JsonMachineDescription = serde_json::from_str(&json_str)
								.expect("Unvalid json data");
	// println!("data : {:?}", json_struct);
	// println!("data transition : {:?}", json_struct.transitions);
	// println!("skip transitions : {:?}", json_struct.transitions["skip"][0]["action"]);

	check_name(&json_struct.name);
	check_validy_letter_alphabet(&json_struct.alphabet);
	check_unicity_alphabet(&json_struct.alphabet.as_slice());
	check_blank(&json_struct.blank, &json_struct.alphabet);
	check_unicity_states(&json_struct.states.as_slice());
	check_initial(&json_struct.initial, &json_struct.states.as_slice());
	check_finals(&json_struct.finals, &json_struct.states);

	println!("Everything works ! (for now...)");
}

fn check_name(name : &str) {
	if name.is_empty() {
		panic!("Empty name");
	}
}

fn check_validy_letter_alphabet(alphabet : &Vec<String>) {
	for letter in alphabet {
		if letter.len() != 1 {
			panic!("{}", "Invalid alphabet, ".to_owned() + letter + " is invalid");
		}
	}
}

fn check_unicity_alphabet(alphabet : &[String]) {
    for i in 1..alphabet.len() {
        if alphabet[i..].contains(&alphabet[i - 1]) {
            panic!("alphabet contains a duplicate letter");
        }
    }
}

fn check_blank(blank : &String, alphabet : &Vec<String>) {
	if blank.len() != 1 {
		panic!("Invalid blank character");
	}
	for letter in alphabet {
		if letter == blank {
			return;
		}
	}
	panic!("blank character is not in the alphabet");
}

fn check_unicity_states(states : &[String]) {
    for i in 1..states.len() {
		if states[i - 1].len() == 0 {
			panic!("empty state");
		}
        else if states[i..].contains(&states[i - 1]) {
            panic!("states contain a duplicate");
        }
    }
}

fn check_initial(initial : &String, states : &[String]) {
	if !states.contains(initial) {
		panic!("Invalid initial state");
	}
}

fn check_finals(finals : &Vec<String>, states : &[String]) {
	for final_state in finals {
		if !states.contains(final_state) {
			panic!("Final state \"{}\" in not in the states", final_state);
		}
	}
}