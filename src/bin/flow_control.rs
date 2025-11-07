use std::thread;

/// author Homeey<jt4mrg@qq.com>
/// 2025-11-07 18:17
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    for i in 1..5 {
        println!("i = {}", i)
    }

    let array = [1, 2, 3, 4, 5];
    for (i, v) in array.iter().enumerate() {
        println!("i = {}, v = {}", i, v)
    }

    for i in &array {
        println!("i = {}", i)
    }

    let handle = thread::spawn(|| {
        println!("hello world");
    });
    handle.join().unwrap();
}
