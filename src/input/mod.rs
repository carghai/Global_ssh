use std::{io, fmt::Display};

pub fn get() -> String {
    let mut return_data = String::new();
    io::stdin()
        .read_line(&mut return_data)
        .expect("Failed to read input");
    return_data
}

pub fn y_n(message : impl Display) -> bool {
    println!("{}", message);
    loop {
    let input = get();
    match input.trim() {
        "y" => {
            return true;
        },
        "n" => {
            return false;
        },
        _ => {
            println!("please repond with y or n")
        },
    }
}

}

