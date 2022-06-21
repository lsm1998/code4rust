#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enums_demo() {
    let p = PokerSuit::Clubs;
    print_suit(p);
    print_suit(PokerSuit::Hearts);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::Write(String::from("lsm"));
    let m4 = Message::ChangeColor(255, 255, 0);
    println!("{:?}-{:?}-{:?}-{:?}", m1, m2, m3, m4);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}
