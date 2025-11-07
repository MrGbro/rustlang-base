fn main() {
    println!("Hello, world!");
    //variable
    variable_bind();
    destructure_pattern();
    variable_shadow();
    infer_type();
    digit_type();
}

fn variable_bind() {
    let x = 5;
    println!("var x = {x}");
    let mut v = 10;
    println!("mutable v = {v}");
    v = 22;
    println!("mutable newest v = {v}");
}

fn destructure_pattern() {
    let (a, mut b) = (false, true);
    println!("a = {a}, b = {b}");
    b = false;
    println!("newest b = {b}");
}

fn variable_shadow() {
    let x = 10;
    println!("outer x = {x}");
    {
        let x = x * 2;
        println!("inner x = {x}");
    }
    println!("outer x again = {x}");
}

fn infer_type() {
    let guess = "42".parse::<i32>().expect("Not a number!");
    println!("guess = {guess}");
}


fn digit_type() {
    let a = 0.2;
    let b: f32 = 0.3;
    println!("a = {a}, b = {b}");
}