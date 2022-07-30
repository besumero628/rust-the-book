fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three")
    }

    println!("{:?}", six); //Some(6)
    println!("{:?}", none); //None
}

