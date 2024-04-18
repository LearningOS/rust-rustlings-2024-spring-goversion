// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

// 切片 &str 虽然可以安全使用，但是，我们很难动态修改其内容 —— 其地址、长度都是固定的。
// 于是 Rust 提供了数据类型 String。String 包含了数据指针、数组容量、数据长度等三个字段。
// 如果新修改的数据长度在其容量范围内，数据可以原地修改。如果新修改的数据长度超出了容量范围，它可以重新申请更大的内存。
// 于是我们看到，String 和 &str 是两个完全不一样的结构体。为什么字符串要保留这两种形式？原因就是效率。
// Rust 希望在数组容量不会变化的时候，用 &str。在数组长度可能发生变化的情况下，使用 String。

/* 
&str或str转String
let s: String = "hello".to_string();
let t: String = String::from("hello");
*/

// 规则很简单，一般情况下，&str 用于只读数据，String 用于可修改的数据。

