/*
 *
 * 实现一个用户输入n，给出斐波那契数列第n项值
 *  
 */


use std::io;

fn main() {
    //读取用户输入
    println!("请输入n的值.");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Fail to read line!");

    let mut number: u32 = number.trim().parse().expect("Input Error");

    println!("The value of number is: {number}");

    let mut result = 0;
    let mut num1 = 1;
    let mut num2 = 1;

    if number==1 || number==2  {
        result = 1;
    }

    while number > 2{
        result = num1 + num2;
        num1 = num2;
        num2 = result;
        number -= 1;
    }

    println!("第n项值为: {result}");
}
