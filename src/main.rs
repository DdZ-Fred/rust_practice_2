enum IpAddress {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    /// The call method is blabla bla
    fn call(&self) {
        println!("blabla {}", "bloblo");
    }
}


fn main() {
    println!("Hello, world!");
    let blabla = IpAddress::V6;

    let x: i8 = 5;
    let y = Some(3); //Typed is unfered, there is no need to use a Option<i8>
    let w = Some("String is blablabla");
    let absent_number: Option<i8> = None;

    let my_message = Message::Write(String::from("Hello"));
    my_message.call();

    let result = x + y.unwrap();

    println!("X + Y = {}", result);
}
