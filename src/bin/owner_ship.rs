fn main() {
    println!("hello world");
    let _a = "name";
    let mut _b = String::from("hello");
    _b.push_str(" world");
    println!("a = {_a},b = {_b}");
    owner_ship_move();
    ref_borrow();
    mutil_borrow();
}

#[allow(unused)]
fn owner_ship_move() {
    let s1 = String::from("hello world");
    let s2 = s1;
    println!("s2 = {s2}");

    let x = "hello world";
    let y = x;
    println!("x ={x},y={y}");

    let a: Point<'_> = Point {
        x: 3,
        y: 4,
        location: "north",
    };
    let b = a;
    println!("a ={:?},b={:?}", a, b);
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point<'a> {
    x: i32,
    y: i32,
    location: &'a str,
}

#[allow(unused)]
fn ref_borrow() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[allow(unused)]
fn mutil_borrow() {
    let mut s = String::from("value");
    let r1 = &mut s;

    println!("r1 = {}", *r1);
}
