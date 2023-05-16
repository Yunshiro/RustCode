fn main() {
    println!("Hello, world!");
    another_func();
    parameters_func(100);
    print_lable(9, 'h');

    //statement带分号,没有返回值,expression才能用来赋值,无分号
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");


    let x = five();
    println!("The value of x is: {x}");

    let sum = add_two(x, y);
    println!("The value of x+y is: {sum}");
}


/*
 * 无返回值的函数 
 */
fn another_func() {
    println!("This is another function.");
}


fn parameters_func(x: i32) {
    println!("The value of x is: {x}");
}

fn print_lable(value: i32, unit_lable: char) {
    println!("The lable is: {value}{unit_lable}");
}

/*
 * 有返回值的函数
 * 包含返回值的表达式不加‘;’
 */
fn five() -> i32 {
    5
}

fn add_two(x: i32, y: i32) -> i32 {
    x+y
}
