use std::io;

fn main() {
    println!("select number");
    let mut select_number = String::new();
    io::stdin()
        .read_line(
            &mut select_number
        ).expect("Error");
    let select_number: u32 = select_number.trim().parse()
        .expect("Please type a number");
    
    let fibo_number: u32 = fibonacci(select_number);
    println!("select number : {}", select_number);
    println!("fibonacci number : {}", fibo_number)
}

fn fibonacci(select_number: u32) -> u32 {
    let mut one_before_number: u32 = 1;
    let mut two_before_number: u32 = 1;
    let mut fibo_number: u32 = 1;

    if select_number < 3 {
        fibo_number = 1;
    } else {
        for _i in 3..=select_number {
            fibo_number = one_before_number + two_before_number;
            one_before_number = two_before_number;
            two_before_number = fibo_number;
        }
    }

    return fibo_number
}