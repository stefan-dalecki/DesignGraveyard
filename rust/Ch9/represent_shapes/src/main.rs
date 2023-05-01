struct Rectangle {
    length: f64,
    width: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.length * self.width
    }

    fn scale(&mut self, scale_value: f64) {
        self.length *= scale_value;
        self.width *= scale_value;
    }

    fn new(length: f64, width: f64) -> Rectangle {
        Rectangle {
            length: length,
            width: width,
        }
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}
