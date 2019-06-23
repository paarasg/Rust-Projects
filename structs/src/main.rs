struct User {
    email: String,
    username: String,
    active: bool,
    account_signin: u8,
}



fn main() {
    let user1 =  User {
        email:String::from("gupta.paaras@gmail.com"),
        username:String::from("guptap35"),
        active: true,
        account_signin: 1,
    };

    let user2 = User {
        ..user1
    };

    println!("{}", user2.email);
}
