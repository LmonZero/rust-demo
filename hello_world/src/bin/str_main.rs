use std::mem;
#[derive(Debug)]
struct A {
    _x: String,
}

fn main() {
    let s = String::from("你好!");
    let x = '好';
    let y = 'a'; //unicode 4

    let z = "a好"; //utf-8 1-4  好3 a1

    println!(
        "{}:{}  {}:{} {}:{}",
        x,
        mem::size_of_val(&x),
        y,
        mem::size_of_val(&y),
        z,
        mem::size_of_val(z)
    );

    let a = A {
        _x: String::from("test"),
    };

    println!("{:?}", a);

    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{} {}", s, mem::size_of_val(s)); //utf8 中文3个字节 string->str 编码不变
}
