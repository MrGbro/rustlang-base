/// author Homeey<jt4mrg@qq.com>
/// 2025-11-07 16:26
fn main() {
    let a = 1;
    println!("{}", a);
    let user = User::build_user("jitao@qq.com".to_string(), "Homeey".to_string());
    println!("user {:?}", user);
    
    let user2 = User {
        username: "Jitao".to_string(),
        ..user
    };
    println!("user2 {:?}", user2)
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    pub fn build_user(email: String, username: String) -> Self {
        Self {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}