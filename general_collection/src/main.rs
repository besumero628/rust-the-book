fn main() {
    let mut v = vec![1,2,3,4,5];

    let first = &v[0];
    println!("The first element is : {:?}", v);
    println!("The first element is : {}", first);

    v.push(6);

    println!("The first element is : {:?}", v);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    println!("The first element is : {:?}", v);
 }

