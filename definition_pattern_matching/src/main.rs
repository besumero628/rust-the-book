fn main() {
    let some_option_value: Option<i32> = Some(5);
    if let Some(x) = some_option_value{
        println!("{}", x);
    };
}
