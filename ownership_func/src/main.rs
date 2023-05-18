fn main() {
    let str = String::from("heart");  // str comes into scope

    takes_ownership(str);        // str's value moves into the function
                                             // ... and so is no longer valid here

    let x = 5;                          // x comes into scope

    makes_copy(x);             // x would move into the function,
                                            // but i32 is Copy, so it's okay to still
                                            // use x afterward
    
    //str已经被释放，会报错
    // println!("The value of str is: {}",str);

    //x 未被释放，还可以使用
    println!("The value of x is: {}",x);



    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}",s2,len);

}   // Here, x goes out of scope, then s. But because str's value was moved, nothing special happens.

    
fn takes_ownership(some_string: String) {
    println!("{}",some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.



fn makes_copy(some_integer: i32) {
    println!("{}",some_integer);
}// Here, some_integer goes out of scope. Nothing special happens.


fn calculate_length(s: String) -> (String,usize) {
    let length = s.len();

    (s, length)
}