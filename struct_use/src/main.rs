struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("jaehoon"),
        email: String::from("uberphoebus@gmail.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("uberphoebus@naver.com");
}
