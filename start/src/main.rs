use std::fmt::Debug;

#[derive(Debug)]
struct Student{
    name: String,
    id:i32
}
fn main() {
    let s=Student{
        name:"asdas".to_string(),
        id:1
    };
    println!("{:?}", s);
    println!();
    println!("Hello, world!");
}