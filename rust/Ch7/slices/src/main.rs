fn main() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    let last_word = &message[15..];
    println!("last word is {}", last_word);
}
