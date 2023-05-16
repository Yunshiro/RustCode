# functions

## 函数声明

使用`fn`关键字

```rust
fn another_func() {
    println!("This is another function.");
}
```

<br>

## 无参函数

```rust
fn main() {
	println!("Hello, world!");
	another_func();
}

fn another_func() {
    println!("This is another function.");
}
```

输出：

```
Hello, world!
This is another function.
```

<br>

## 有参函数

```rust
fn main() {
    print_lable(9, 'h');
}

fn print_lable(value: i32, unit_lable: char) {
    println!("The lable is: {value}{unit_lable}");
}
```

输出：

```
The lable is: 9h
```

<br>

## 有返回值的函数



```rust
fn main() {
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

fn five() -> i32 {
    5
}

fn add_two(x: i32, y: i32) -> i32 {
    x+y
}
```

输出：

```
The value of y is: 4
The value of x is: 5
The value of x+y is: 9
```



返回值那一行语句是没有分号结束的，在rust中，以分号结束的称为`statements`，是没有返回值的，`expressions`则有返回值，无分号结尾。

