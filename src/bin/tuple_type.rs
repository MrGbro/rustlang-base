/// author Homeey<jt4mrg@qq.com>
/// 2025-11-07 15:22
fn main() {
    println!("system out errors");

    let tuple = (1, 2.1, 3);
    println!("tuple: {:?}", tuple);

    println!("tuple: {:?}", tuple.0);
    println!("tuple: {:?}", tuple.1);
    println!("tuple: {:?}", tuple.2);

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
    //          too_long_tuple.0, too_long_tuple.1, too_long_tuple.2, too_long_tuple.3,
    //          too_long_tuple.4, too_long_tuple.5, too_long_tuple.6, too_long_tuple.7,
    //          too_long_tuple.8, too_long_tuple.9, too_long_tuple.10, too_long_tuple.11,
    //          too_long_tuple.12);
}
