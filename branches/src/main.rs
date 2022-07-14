fn main() {
    let mut count = 0;
    'counting_up: loop{
        println!("count = {}", count);
        let mut reamining = 10;

        loop {
            println!("remaining = {}", reamining);
            if reamining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            reamining -=1;
        }
        count += 1;
    }
    println!("End count = {}", count)
}