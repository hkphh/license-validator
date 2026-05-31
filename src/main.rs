use regex::Regex;
use std::io;

// xxxx-xxxx-xxxx
// 14 chars
// the key format

fn main() {
    let mut user_input = String::new();

    println!("Input the key!");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    println!("You typed {user_input}");
    if valid_len(&user_input) {
        println!("Good len");
        if valid_pref(&user_input) {
            println!("Good pref")
        } else {
            println!("Bad pref")
        }
        if valid_digits(&user_input) {
            println!("Good digits")
        } else {
            println!("Bad digits")
        }
    } else {
        println!("Bad len")
    }
}

fn valid_len(in_key: &String) -> bool {
    let user_input_len = in_key.chars().count() - 1;
    println!("The lenght is {user_input_len}");
    match user_input_len {
        14 => true,
        _ => false,
    }
}

fn valid_pref(in_key: &String) -> bool {
    let test = in_key.as_bytes()[13] as char;

    let index_f = 4;
    let index_s = 9;
    let fp = in_key.as_bytes()[index_f] as char;
    let sp = in_key.as_bytes()[index_s] as char;
    if fp == '-' && sp == '-' {
        true
    } else {
        false
    }
}

fn valid_digits(in_key: &String) -> bool {
    let re = Regex::new(r"\d{4}-\d{4}-\d{4}").unwrap();
    if re.is_match(in_key) {
        true
    } else {
        false
    }
}
