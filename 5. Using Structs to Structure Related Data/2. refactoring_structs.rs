
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rectangle = Rectangle{width: 50, height: 30};

    let area_result = area(&rectangle);
    println!("Area for rectangle is {area_result}")
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
    