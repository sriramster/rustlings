#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Resize {
    width: u32,
    height: u32,
}

struct Move;
struct Echo;
struct Quit;
struct ChangeColor (i32, i32, i32);

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize{width: u32, height: u32},
    Move(Point),
    Echo(String),
    Quit,
    ChangeColor(i32, i32, i32),

}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
