use std::fmt ::Display;

fn longenst_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "thie is s1 sentence";
    let s2 = "thie is s2 sentence";

    longenst_with_an_announcement(s1, s2, 5);
}