#![allow(dead_code, unused)]

use std::io;
use std::collections::HashMap;
use std::process;

enum ConversionType {
    LENGTH,
    MASS,
}

enum LengthType {
    MM,
    CM,
    M,
    KM,
    MILE,
    YARD,
    INCH,
    FOOT,
}

struct Conversion {
    val: f32,
    c_type: ConversionType,
}

const CONVERTION_TYPES: &'static [&'static str] = &["Length", "Mass"];

const LENGTH_TYPES: &'static [&'static str] =
    &["MM", "CM", "M", "KM", "MILE", "YARD", "INCH", "FOOT"];

const MASS_TYPES: &'static [&'static str] = &["KG", "G", "MG", "POUND", "OUNCE"];

fn convert_length(_val: f32) -> f32 {
    return 1.1;
}

fn converter_factory(conversion_type: &str) -> impl Fn(f32) -> f32 {
    match conversion_type {
        "Length" => return convert_length,
        "Mass" => print!("mass"),
        &_ => (),
    }
    return convert_length;
}

fn ExitOnError(msg: &str) {
    println!("Error: {}", msg);
    process::exit(1)
}

fn main() -> io::Result<()> {
    let value: String = std::env::args().nth(1).expect("No value provided");
    let f_val: f32 = value.as_str().parse::<f32>().unwrap();

    // let f_val: value. parse::f32();
    let conversion_types: [&str; 2] = ["Length", "Mass"];
    let mut index_to_type = HashMap::from([(1, "Length"), (2, "Mass")]);

    println!("What do you want to convert?");
    for i in 0..conversion_types.len() {
        match conversion_types.get(i) {
            Some(val) => println!("{}. {}", i + 1, val),
            None => ExitOnError(""),
        }
    }
    
    let mut c_choice = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut c_choice);

    let mut i_c_choice = -1; 
    match c_choice.trim().parse::<i32>() {
        Ok(val) => i_c_choice = val,
        Err(..) => ExitOnError("Invalid Choice"),
    }

    if i_c_choice > conversion_types.len() as i32 {
        ExitOnError("Invalid Choice")
    }

    let mut c_conv_f: &str = "";
    match index_to_type.get(&i_c_choice) {
        Some(v) => c_conv_f = v,
        None => ()
    }
    
    print!("Converting {}", c_conv_f);
    converter_factory(c_conv_f)(f_val);
    

    Ok(())
}
