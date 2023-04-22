fn main() {
    let a: bool = true;
    let b: bool = true;
    let c: bool = false;
    println!("XOR {}", a ^ b); // false if same
    let d: bool = a || b;
    println!("short circuit {d}");
    println!("short circuit {}", c && d);
}
