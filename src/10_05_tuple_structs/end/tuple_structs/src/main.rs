struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    let my_color = Color(255, 0, 0);
    println!("The first value of my_color is {}", my_color.0);

    let my_point = Point(4, 5, 6);
    let y = get_y(my_point);
    println!("y is {y}");
}
