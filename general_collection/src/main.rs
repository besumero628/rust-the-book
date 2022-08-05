fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s4 = String::from("Goodbey");
    let s5 = String::from("earth!");
    let s6 = format!("{} - {}", s4, s5);

    println!("{}", s);
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s4);
    println!("{}", s5);
    println!("{}", s6);

    let hello = "Здравствуйте";

    let s7 = &hello[0..4];
    println!("{}", s7);
}
