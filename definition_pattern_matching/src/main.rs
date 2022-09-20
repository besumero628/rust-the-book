fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x2 = Some(5);
    let y2 = 10;

    match x2 {
        Some(50) => println!("Got 50"),
        Some(y2) => println!("Matched, y = {:?}", y2),
        _ => println!("Default case, x2 = {:?}", x),
    }

    println!("at the end: x2 = {:?}, y2 = {:?}", x2, y2);

    let x3 = 1;

    match x3 {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
