use std::hash::Hash;

fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);
    // println!("{:?}", field_name); この時点でmapに所有権がmoveしている
}
