use std::fs;

fn main() {
    let file_name = &String::from("planets.txt");

    let contents = fs::read_to_string(file_name).unwrap();

    for line in contents.lines() {
        println!("line is {}", line);
    }

    let byte_contents = fs::read(file_name).unwrap();
    println!("contents is {:?}", byte_contents);
}
