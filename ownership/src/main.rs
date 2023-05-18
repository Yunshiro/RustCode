fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    {
        let str = String::from("I love coding!");
        println!("inner scope: {}",str);
    }  //Rust calls drop automatically at the closing curly bracket.

    // println!(str); 会报错，str出了scope之后，GC自动回收str


    // let s1 = String::from("love");
    // let s2 = s1;
    // println!("{}, you", s1);
    // 会报错，s1 move into s2 只有s2是有效的


}


