use std::io;

/// author Homeey<jt4mrg@qq.com>
/// 2025-11-07 17:46
fn main() {
    read_input();
}

fn read_input() {
    let a = [1, 2, 3, 4, 5];
    println!("Please input an array index:");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let index: usize = s.trim()
        .parse()
        .expect("Please type a number");
    let element = a[index];
    println!("The value of element is: {}", element);
}


#[allow(unused)]
fn simple_array() {
    let a = [1, 2, 3];
    println!("{:?}", a);
    
    let a = [0; 5];
    println!("{:?}", a);
    
    let a = "hello world";
    let b = &a[0..1];
    println!("{:?}", b);
}