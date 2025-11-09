#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor(u8, u8, u8), 
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor(100, 100, 100));
    println!("{:?}", Message::Quit);
}
