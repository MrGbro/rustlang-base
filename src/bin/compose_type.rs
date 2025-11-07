fn main() {
    println!("compose type");
    slice_usage();
    string_append();
    string_append_2();
    retrive_chars();
    exercise::strs();
    exercise::string_append();
}

fn slice_usage() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("hello value is {hello}");

    let source = "中国人";
    println!("source len is {:?}", source.len());

    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is {word}");
    s.clear();

    let a = "hello world".to_owned();
    println!("string {a}");

    let rep = String::from("hello world");
    let result = rep.replace("e", "s");
    println!("res {result}");
}

#[allow(unused)]
fn first_word(s: &String) -> &str {
    &s[0..1]
}

fn string_append() {
    let a = "s".to_string();
    let b = "m".to_string();
    let c = b + &a;
    println!("c is {c}");
}

fn string_append_2() {
    let a = "hello";
    let b = String::from("world");
    let c = format!("{} {}!", a, b);
    println!("c is {c}");
    println!("a is {a}");
}

fn retrive_chars() {
    let s = "中国人";
    for c in s.chars() {
        println!("char is {c}");
    }
}

mod exercise {
    #[allow(unused)]
    pub fn strs() {
        let s: Box<str> = "hello world".into();
        greet(&s);
        greet(s.as_ref());
    }

    fn greet(s: &str) {
        println!("hello {s}");
    }

    #[allow(unused)]
    pub fn string_append() {
        let mut s = String::new();
        s.push_str("hello, world");
        s.push('!');
        assert_eq!(s, "hello, world!");
    }
}
