struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let white = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("white({},{},{})",white.0,white.1,white.2);
    println!("orgin({},{},{})",origin.0,origin.1,origin.2);
}
