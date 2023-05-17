fn main() {
    //循环语句 loop while for
    
    //loop 其中的break会结束循环并且返回后面的值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");


    println!("---------------------------");
    //loop标签，用于消除多重loop的歧义
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;     //结束外围带'counting_up标签的循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    println!("---------------------------");

    //while循环
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");


    println!("---------------------------");
    //for循环
    let a = [1,2,3,4,5];
    for element in a {
        println!("the value is: {element}");
    }
    
    println!("---------------------------");
    //rev逆序（1..4）
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!");
}
