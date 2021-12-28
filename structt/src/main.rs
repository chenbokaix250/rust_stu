struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("abc@123.com"),
        username: String::from("Niky"),
        active: true,
        sign_in_count: 566,
    };

    println!("email:{}",user1.email);
}
