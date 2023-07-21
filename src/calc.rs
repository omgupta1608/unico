use std::process;

pub fn calculate_conversion(c_type: &str, val: f32, from: &str, to: &str) -> f32 {
    if from == to {
        return  val;
    }

    let conversion_case = format!("{}|{}", from, to);
    let mut result: f32 = -1.1;
    match  c_type {
        "Length" => result = calc_length(&conversion_case, val),
        "Mass" => result = calc_mass(&conversion_case, val),
        _ => ()
    }

    if result == -1.1 {
        println!("Something went wrong");
        process::exit(1)
    }

    result
}

fn calc_length(conv: &str, val: f32) -> f32 {
    match conv {
        "MM|CM" => (val / 10.0),
        "MM|M" => (val / 1000.0),
        "MM|KM" => (val / 1000000.0),
        "CM|MM" => (val * 10.0),
        "CM|M" => (val / 100.0),
        "CM|KM" => (val / 100000.0),
        "M|KM" => (val / 1000.0),
        "M|MM" => (val * 1000.0),
        "M|CM" => (val * 100.0),
        "KM|MM" => (val * 1000000.0),
        "KM|CM" => (val * 100000.0),
        "KM|M" => (val * 1000.0),
        &_ => (-1.1)
    }
}

fn calc_mass(conv: &str, val: f32) -> f32 {
    match conv {
        "KG|G" => (val * 1000.0),
        "KG|MG" => (val * 1000000.0),
        "G|KG" => (val / 1000.0),
        "G|MG" => (val * 1000.0),
        "MG|KG" => (val / 1000000.0),
        "MG|G" => (val / 1000.0),
        &_ => (-1.1)
    }
}