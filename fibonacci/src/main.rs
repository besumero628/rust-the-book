use std::io;

fn main() {
    println!("Hello, world!");

    let mut select_number = String::new();
    io::stdin()
        .read_line(
            &mut select_number
        ).expect("Error");
    let select_number: u32 = select_number.trim().parse()
        .expect("Please type a number");

    println!("in : {}", select_number)
}
