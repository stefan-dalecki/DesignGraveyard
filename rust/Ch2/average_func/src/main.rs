fn main() {
    let a: u8 = 13;
    let b: f64  = 2.3;
    let c: f32 = 120.0;

    let average: f64 = (a as f64+b+c as f64) / 3.0;
    assert_eq!(average, 45.1);
    println!("passed");
}
