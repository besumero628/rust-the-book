mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restrant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::add_to_waitlist();
}