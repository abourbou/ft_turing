
use std::fs;
use serde::{Deserialize, Serialize};
extern crate serde_json;
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

#[derive(Debug)]
pub struct Transition {
	pub read : char,
	pub to_state : String,
	pub write : char,
	pub go_right : bool,
}

#[derive(Debug)]
pub struct MachineDescription {
	pub name: String,
	pub blank: char,
	pub initial: String,
	pub finals: Vec<String>,
	pub list_transitions: HashMap<String, Vec<Transition>>,
}

pub fn parse_json(path : &str) -> MachineDescription {

	let error_path = "Couldn't find or load \"".to_owned() + path + "\" json";
	let json_str = fs::read_to_string(path)
						.expect(&error_path);

	let json_struct: JsonMachineDescription = serde_json::from_str(&json_str)
								.expect("Unvalid json data");
	check_validity_json(&json_struct);

	let final_description : MachineDescription = create_description(json_struct);

	final_description
}

fn check_validity_json(json_struct: &JsonMachineDescription) {
	check_name(&json_struct.name);
	check_validy_letter_alphabet(&json_struct.alphabet);
	check_unicity_alphabet(json_struct.alphabet.as_slice());
	check_blank(&json_struct.blank, &json_struct.alphabet);
	check_unicity_states(json_struct.states.as_slice());
	check_initial(&json_struct.initial, json_struct.states.as_slice());
	check_finals(&json_struct.finals, &json_struct.states);
	check_transitions(&json_struct.alphabet, &json_struct.states, &json_struct.transitions);

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

fn check_transitions(alphabet : &[String], states : &[String], transitions : &HashMap<String, Value>) {
	transitions.iter().for_each(|transition| {
		if !states.contains(transition.0) {
			panic!("Invalid transition \"{}\"", transition.0);
		}

		let arr_transition = transition.1.as_array().expect("Invalid transition");
		arr_transition.iter().for_each(|elem_arr_transition| {
			let obj_transition = elem_arr_transition.as_object().expect("Invalid transition");
			check_transition(alphabet, states, obj_transition)
		});
	});
}

fn check_transition(alphabet : &[String], states: &[String], transition : &serde_json::Map<String, Value>) {
	if !transition.contains_key("read") || !transition.contains_key("to_state") ||
		!transition.contains_key("write") || !transition.contains_key("action"){
		panic!("Invalid transition");
	}

	if !alphabet.contains(&transition["read"].as_str().expect("Invalid transition").to_string()){
		panic!("Invalid transition");
	}

	if !states.contains(&transition["to_state"].as_str().expect("Invalid transition").to_string()) {
		panic!("Invalid transition");
	}

	if !alphabet.contains(&transition["write"].as_str().expect("Invalid transition").to_string()){
		panic!("Invalid transition");
	}

	if transition["action"] != "LEFT" && transition["action"] != "RIGHT" {
		panic!("Invalid transition");
	}
}


fn create_description(json_struct: JsonMachineDescription) -> MachineDescription {

	MachineDescription {
		name: json_struct.name,
		blank: json_struct.blank.chars().next().unwrap(),
		initial: json_struct.initial,
		finals: json_struct.finals,
		list_transitions: json_struct.transitions.iter().map(|transi| {
			(transi.0.clone(),
			transi.1.as_array().unwrap()
				.iter().map(|transition| {
					let obj_transi = transition.as_object().unwrap();
					Transition {read: 		obj_transi["read"].as_str().unwrap().chars().next().unwrap(),
								to_state:	obj_transi["to_state"].as_str().unwrap().to_string(),
								write:		obj_transi["write"].as_str().unwrap().chars().next().unwrap(),
								go_right:	obj_transi["action"].as_str().unwrap() == "RIGHT",
							}
				}).collect()
			)
			}).collect(),
	}
}
