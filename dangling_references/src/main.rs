fn main() {
    let reference_to_nothing = dangle();
    println!("{}",reference_to_nothing);
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s  // we return a reference to the String, s
// }// Here, s goes out of scope, and is dropped. Its memory goes away.
//  // Danger

//上面因为s未被使用，会将s内存自动释放，所以会找不到引用，
//rust不允许传入空指针，而下面s会返回给reference_to_nothing，s内存不会被释放
fn dangle() -> String {
    let s = String::from("hello");
    s
}
