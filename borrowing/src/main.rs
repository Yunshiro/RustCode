fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}",s);

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{},{}",r1,r2);
    //we cannot borrow `s` as mutable more than once at a time.


    
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}






