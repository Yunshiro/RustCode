# References and Borrowing



## 可变引用



```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

输出结果：

```rust
   Compiling borrowing v0.1.0 (/home/cookie/project/borrowing)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
7 | fn change(some_string: &mut String) {
  |                        ~~~~~~~~~~~

For more information about this error, try `rustc --explain E0596`.
error: could not compile `borrowing` due to previous error
```

被引用的值如果不可变，引用是不可以修改的

<br>

修改后：

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}",s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

为被引用变量加上`mut`，函数参数也需要加上`mut`

<br>

如下代码：

```rust
fn main() {
    //we cannot borrow `s` as mutable more than once at a time.
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{},{}",r1,r2);
}
```

会报错，不能引用可变变量多于一次。



<br>



当有不可变的引用时，不能有可变的引用指向同一个值。代码如下：

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```



