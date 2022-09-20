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

    let x4 = 5;
    match x4 {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }

    let x5 = 'c';

    match x5 {
        // ASCII文字前半
        'a'..='j' => println!("early ASCII letter"),
        // ASCII文字後半
        'k'..='z' => println!("late ASCII letter"),
        // それ以外
        _ => println!("something else"),
    }


    let num = Some(4);

    match num {
        // 5未満です: {}
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            // 範囲内のidが見つかりました: {}
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            // 別の範囲内のidが見つかりました
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            // それ以外のidが見つかりました
            println!("Found some other id: {}", id)
        },
    }
}
