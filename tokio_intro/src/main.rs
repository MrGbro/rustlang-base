// fn main() {
//     println!("Hello, world!");
//     let runt = runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();
//     runt.block_on(hi());
// }

use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // tokio::join!(hello2(1, 200),hello2(2, 200),hello2(3, 200));
    // let _ = tokio::join!(
    //     tokio::spawn(hello2(1, 200)),
    //     tokio::spawn(hello2(2, 200)),
    //     tokio::spawn(hello2(3, 200))
    // );

    let _ = tokio::join!(delay(1, 200), delay(2, 200), delay(3, 200));
}

#[allow(unused)]
async fn hi() {
    println!("hi")
}
#[allow(unused)]
async fn hello() {
    println!("hello")
}
#[allow(unused)]
async fn run() {
    for i in 0..10 {
        println!("{}", i);
    }
}

#[allow(unused)]
async fn add(x: i32, y: i32) -> i32 {
    println!("add function called with {} and {} equals {}", x, y, x + y);
    x + y
}

#[allow(unused)]
async fn hello2(task: u64, time: u64) {
    println!(
        "Task {task} started, will take {time} ms use {:?}",
        thread::current().id()
    );
    tokio::time::sleep(Duration::from_millis(time)).await;
    println!("Task {task} done use {:?}", thread::current().id());
}

#[allow(unused)]
async fn delay(task: u64, time: u64) {
    println!("Task {task} started");
    tokio::task::spawn_blocking(move || {
        thread::sleep(Duration::from_millis(time));
        println!("Task {task} done use {:?}", thread::current().id());
    });
    println!("Task {task} ends {:?}", thread::current().id());
}
