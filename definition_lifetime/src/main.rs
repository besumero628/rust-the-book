struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "test";

    {
        let s1: &'static str = "test";

        println!("{}",s1);
    }

    println!("{}",s);
    
}