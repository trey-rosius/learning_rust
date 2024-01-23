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
    println!(" user id is {} and email is {}", stream.user_id, stream.email); 


}
