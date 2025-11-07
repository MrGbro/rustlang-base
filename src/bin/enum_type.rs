/// author Homeey<jt4mrg@qq.com>
/// 2025-11-07 17:10
fn main() {
    let a = 1;
    println!("{}", a);
    let five = Some(5);
    println!("{:?}", plus_one(five));
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None=>None,
        Some(i)=>Some(i+1),
    }
}