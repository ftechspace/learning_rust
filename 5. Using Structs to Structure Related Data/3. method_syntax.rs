

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle{width: 50, height: 30};

    let area_result = rectangle.area();
    println!("Area for rectangle is {area_result}");
}


    