// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

fn main() {
    let shipping_box = ShippingBox::new();
    shipping_box.print();
}

#[derive(Debug)]
enum Color{
    Red,
    Green,
    Blue
}
impl Color {
    fn print(&self) {
        let color = match self {
            Color::Blue => "blue",
            Color::Red => "red",
            Color::Green => "green"
        };

        println!("color: {}", color);
    }
}


#[derive(Debug)]
struct Dimension {
    height_mm: f64,
    width_mm: f64,
    length_mm: f64
}
impl Dimension{
    fn print(&self) {
        println!("height: {}mm", self.height_mm);
        println!("width: {}mm", self.width_mm);
        println!("length: {}mm", self.length_mm);
    }
}

#[derive(Debug)]
struct ShippingBox {
    color: Color,
    weight_mg: i32,
    dimensions: Dimension
}

impl ShippingBox{
    fn new() -> Self {
        Self { color: Color::Green, weight_mg: 500, dimensions: Dimension { height_mm: 12000.0, width_mm: 12000.0, length_mm: 15000.0 } }
    }

    fn print(&self) {
        println!("");
        print!("ShippingBox(");
        println!("weight: {}mg", self.weight_mg);
        self.color.print();
        self.dimensions.print();
        print!(")");
        println!("");
    }
}
