# Rust学习日志
> 笔记上传至notebook中


## 2023-5-15

### cargo

cargo文档组成：

1. Cargo.toml: 项目版本，名字，所用依赖

2. Cargo.lock: 保存上一次build的版本，依赖之类的

3. src目录: 存放源代码

4. target目录: 生成的文件



常用命令：

1. 创建项目

```
$ cargo new hello_cargo
```



2. 运行项目

```
$ cd hello_cargo
$ cargo run
```



3. 检查和构建项目

```
$ cargo check
$ cargo build
```



