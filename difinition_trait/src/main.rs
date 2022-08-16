use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl Pair<i32> {
    fn plus(&self) -> i32 {
        self.x + self.y
    }
}

fn main() {
    let p1 = Pair::new(5,6);
    let p2 = Pair::new("y", "x");

    p1.cmp_display();
    p2.cmp_display();

    println!("{}", p1.plus());
}

