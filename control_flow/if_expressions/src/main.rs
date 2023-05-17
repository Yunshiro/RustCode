fn main() {
    //if expressions
    let number = 9;
    if number < 5 {
        println!("condition was true");
    } else if  number==5{
        println!("number is five");
    } else {
        println!("condition was false");
    }


    /*
     *  let number = 3;
     *
     *  if number {
     *       println!("number was three");
     *  }
     *  会报错，rust不会自动将数值型的转换成布尔型
     */


    //using if in a let statement
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
