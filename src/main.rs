#[derive(Debug)]
struct Stream {
    user_id: String,
    email: String,
}
fn main() {
    let stream = Stream {
        user_id: String::from("1"),
        email: String::from("1"),
    };
    println!("{:?}", stream);
    let stream2 = Stream {
        user_id: String::from("1"),
        email: String::from("1"),
    };
    println!("{:?}", stream2);
    let stream3 = Stream {
        user_id: String::from("1"),
        email: String::from("1"),
    };
    println!("{:?}", stream3);
    let stream4 = Stream {
        user_id: String::from("1"),
        email: String::from("1"),
    };
    println!("{:?}", stream4);
    let stream5 = Stream {
        user_id: String::from("1"),
        email: String::from("1"),
    };
    println!("{:?}", stream5);
    let stream6 = Stream {
        user_id: String::from("1"),
        email: String::from("1"),
    };
    println!("Hello, world!");
}
