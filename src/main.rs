#![allow(dead_code)]

use std::io;

enum ConversionType {
    LENGTH,
    MASS,
    // ANGLE,
    // AREA,
    // VOLUME,
}

// enum 

fn convert(_val: f32) -> f32 {
    return 1.1
}

fn converter_factory(conversion_type: ConversionType) -> impl Fn(f32) -> f32 {
    match conversion_type {
        ConversionType::LENGTH => print!("Length"),
        ConversionType::MASS => print!("mass")
    }
    return convert;
}


fn main() -> io::Result<()>{
    // let pattern: String = std::env::args().nth(1).expect("no pattern given");
    // print!("{}", pattern)
    let mut a = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut a);
    print!("{}", a);
    Ok(())
}
