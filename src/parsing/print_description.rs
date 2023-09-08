
use super::parse_json::*;

pub fn print_description(description : &MachineDescription) {

    println!("
********************************************************************************
* *
* {} *
* *
********************************************************************************", description.name);

    print!("Alphabet: [");
    for letter in description.alphabet.iter() {
        print!("{}", letter);
        if letter != description.alphabet.iter().last().unwrap() {
            print!(", ");
        }
    }
    println!(" ]", );

    print!("States: [ ");
    for state_name in description.states.iter() {
        print!("{}", state_name);
        if state_name != description.states.iter().last().unwrap() {
            print!(", ");
        }
    }
    println!(" ]", );

    println!("Initial : {}", description.initial);
    
    print!("Finals : [ ");
    for final_state in description.finals.iter() {
        print!("{}", final_state);
        if final_state != description.finals.iter().last().unwrap() {
            print!(", ");
        }
    }
    println!(" ]");

    for list_transition in description.list_transitions.iter() {
        let state_name = &list_transition.0;
        for transition in list_transition.1 {
            print!("({}, {}) -> ({}, {}, ", state_name, transition.read, transition.to_state, transition.write);
            if transition.go_right {
                println!("RIGHT)");
            }
            else {
                println!("LEFT)");
            }
        }
    }
    println!("********************************************************************************");
}