
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

	let error_path = "Couldn't find or load \"".to_owned() + path + "\" json";
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
	check_unicity_alphabet(json_struct.alphabet.as_slice());
	check_blank(&json_struct.blank, &json_struct.alphabet);
	check_unicity_states(json_struct.states.as_slice());
	check_initial(&json_struct.initial, json_struct.states.as_slice());
	check_finals(&json_struct.finals, &json_struct.states);

	println!("Everything works ! (for now...)");
}

fn check_name(name : &str) {
	if name.is_empty() {
		panic!("Empty name");
	}
}

fn check_validy_letter_alphabet(alphabet : &[String]) {
	if let Some(letter) = alphabet.iter().find(|&letter| letter.len() != 1) {
		panic!("{}", "Invalid alphabet, \"".to_owned() + letter + "\" is invalid");
	}
}

fn check_unicity_alphabet(alphabet : &[String]) {

	fn check_duplicate_alphabet_rec(alphabet : &[String]) -> bool{
		if alphabet.is_empty() {
			return false;
		} else if alphabet[1..].contains(&alphabet[0]) {
			return true;
		}
		check_duplicate_alphabet_rec(&alphabet[1..])
	}

	if check_duplicate_alphabet_rec(alphabet) {
		panic!("alphabet contains a duplicate letter");
	}
}

fn check_blank(blank : &String, alphabet : &[String]) {
	if blank.len() != 1 {
		panic!("Invalid blank character");
	}
	if !alphabet.iter().any(|letter| letter == blank) {
		panic!("blank character \"{}\" is not in the alphabet", blank);
	}
}

fn check_unicity_states(states : &[String]) {
	if states.iter().any(|state| state.is_empty()) {
		panic!("empty state");
	}

	fn check_duplicate_state_rec(states : &[String]) -> bool{
		if states.is_empty() {
			return false;
		} else if states[1..].contains(&states[0]) {
			return true;
		}
		check_duplicate_state_rec(&states[1..])
	}

	if check_duplicate_state_rec(states) {
		panic!("states contain a duplicate");
	}
}

fn check_initial(initial : &String, states : &[String]) {
	if !states.contains(initial) {
		panic!("Invalid initial state");
	}
}

fn check_finals(finals : &[String], states : &[String]) {
	if let Some(bad_final_state) = finals.iter().find(|&final_state| !states.contains(final_state)) {
		panic!("Final state \"{}\" in not in the states", bad_final_state);
	}
}