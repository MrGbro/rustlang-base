use futures::executor::block_on;
use futures::join;

// 1.async function 可以执行non-async函数
// 2. non-async函数不能执行async函数,除非有executor
fn main() {
    block_on(do_mul());
}

fn hello_sync() {
    println!("hello async!");
}

async fn hello() {
    println!("Hello world!");
    hello_sync();
}

async fn hi() {
    println!("Hi!");
    hello().await;
}

async fn do_mul() {
    join!(hi(),hello());
    let sum = add(1, 2).await;
    println!("{}", sum);

    let (a,b) = join!(add(1, 2), add(3, 4));
    println!("Sum: {a} {b}");
}

async fn add(a: i32, b: i32) -> i32 {
    a + b
}
