#![allow(dead_code, unused)]

use std::io;
use std::collections::HashMap;
use std::process;
use std::ptr;

const CONVERSION_TYPES: [&str; 2] = ["Length", "Mass"];

const LENGTH_TYPES: &'static [&'static str] =
    &["MM", "CM", "M", "KM", "MILE", "YARD", "INCH", "FOOT"];

const MASS_TYPES: &'static [&'static str] = &["KG", "G", "MG", "POUND", "OUNCE"];

fn convert_length(_val: f32) -> f32 {

    println!("From? :");
    for i in 0..LENGTH_TYPES.len() {
        match LENGTH_TYPES.get(i) {
            Some(val) => println!("{}. {}", i + 1, val),
            None => exit_on_error("Something went wrong"),
        }
    }

    let mut from_choice = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut from_choice);
    from_choice = from_choice.trim().to_string();

    println!("To? :");
    for i in 0..LENGTH_TYPES.len() {
        match LENGTH_TYPES.get(i) {
            Some(val) => println!("{}. {}", i + 1, val),
            None => exit_on_error("Something went wrong"),
        }
    }

    let mut to_choice = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut to_choice);
    to_choice = to_choice.trim().to_string();

    println!("From {} to {}", from_choice, to_choice);
    return 1.1;
}

fn convert_mass(_val: f32) -> f32 {
    println!("From? :");
    for i in 0..MASS_TYPES.len() {
        match MASS_TYPES.get(i) {
            Some(val) => println!("{}. {}", i + 1, val),
            None => exit_on_error("Something went wrong"),
        }
    }

    let mut from_choice = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut from_choice);
    from_choice = from_choice.trim().to_string();

    println!("To? :");
    for i in 0..MASS_TYPES.len() {
        match MASS_TYPES.get(i) {
            Some(val) => println!("{}. {}", i + 1, val),
            None => exit_on_error("Something went wrong"),
        }
    }

    let mut to_choice = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut to_choice);
    to_choice = to_choice.trim().to_string();

    println!("From {} to {}", from_choice, to_choice);
    return 1.1;
}

fn converter_factory(conversion_type: &str) -> impl Fn(f32) -> f32 {
    println!("{}", conversion_type);
    match conversion_type {
        "Length" => convert_length,
        "Mass" => convert_mass,
        &_ => {
            println!("Invalid choice. Defaulting to Length");
            convert_length
        },
    }
}

fn exit_on_error(msg: &str) {
    println!("Error: {}", msg);
    process::exit(1)
}

fn main() -> io::Result<()> {
    let value: String = std::env::args().nth(1).expect("No value provided");
    let f_val: f32 = value.as_str().parse::<f32>().unwrap();

    println!("What do you want to convert?");
    for i in 0..CONVERSION_TYPES.len() {
        match CONVERSION_TYPES.get(i) {
            Some(val) => println!("{}. {}", i + 1, val),
            None => exit_on_error("Something went wrong"),
        }
    }

    let mut c_choice = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut c_choice);

    let mut i_c_choice = -1; 
    match c_choice.trim().parse::<i32>() {
        Ok(val) => i_c_choice = val,
        Err(..) => exit_on_error("Invalid Choice"),
    }

    if i_c_choice > CONVERSION_TYPES.len() as i32 {
        exit_on_error("Invalid Choice")
    }

    let mut c_conv_f: &str = CONVERSION_TYPES[(i_c_choice - 1) as usize];

    println!("Converting {}...", c_conv_f);
    converter_factory(c_conv_f)(f_val);
    
    Ok(())
}
