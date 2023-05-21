fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);      //word的值为5
    s.clear();                             //清空字符串s，变成 ""

    println!("The length of first word is: {}",word);


    let str = String::from("big value");
    let word1 = first_word_2(&str);
    println!("{}",word1);


}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
