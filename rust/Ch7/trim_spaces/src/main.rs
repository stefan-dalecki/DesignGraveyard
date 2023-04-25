fn main() {
    input_str = String::from("   3 leading spaces, 2 trailing spaces  ")
}

fn trim_spaces(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &value) in bytes.iter().enumerate() {
        if value != b' ' {
            let &s = &s[..index]
        }
    }
    for (index, &value) in bytes.iter().rev().enumerate() {
        if value != b' ' {
            let &s = &s[index..]
        }
    }
    return &s;
}
