#[derive(Debug)]
enum Message {
    Move,
    Resize,
    Echo,
    ChangeColor,
    Quit
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
