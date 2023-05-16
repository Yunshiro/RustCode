fn main() {
    /*
     * 变量默认是immutable不可变，需要加上mut 
     */

    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    /*
     * x在{}中，变成6*2=12，当这个scope结束后，x会重新变为6
     */

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    /*
     * 第一个spaces是String类型,第二个是数值类型的,会发生错误
     */
    // let mut spaces = "    ";
    // spaces = spaces.len();

}
