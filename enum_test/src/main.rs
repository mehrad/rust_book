#[derive(Debug)]
enum Message { 
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("ChangeColor to r: {}, g: {}, b: {}", r, g, b),
        }
    } 
}
fn main() {
    let mut m = Message::Write(String::from("Hello"));
    m.call();
    m = Message::Move { x: 1, y: 2 };
    m.call();
    Message::ChangeColor(255, 255, 255).call();
}
