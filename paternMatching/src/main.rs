enum Rectangle{
    Area(f32, f32),
    Perimeter(f32, f32),
}

fn find(shap: Rectangle) -> f32{
    match shap {
        Rectangle::Area(num1, num2) => num1 * num2,
        Rectangle::Perimeter(num1, num2) => 2.0 * (num1 + num2),
    }
}

fn main() {
    let area = Rectangle::Area(2.00, 2.00);
    let perimeter = Rectangle::Perimeter(2.00, 2.00);

    println!("Area: {}", find(area));
    println!("Perimeter: {}", find(perimeter));
}