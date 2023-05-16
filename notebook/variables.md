# Variables

## 变量声明

变量用`let`关键字声明,变量默认是immutable，不可变的，需要加上`mut`关键字

```rust
let mut x = 5;
println!("The value of x is: {x}");
x = 6;
println!("The value of x is: {x}");
```

<br>



## scope

`{}`花括号可以代表是一个scope，花括号结束后scope结束，x会重新变回进入scope之前的值`6`。

```rust
let x = 5;
let x = x + 1;
{
	let x = x * 2;
	println!("The value of x in the inner scope is: {x}");
}

	println!("The value of x is: {x}");
```

