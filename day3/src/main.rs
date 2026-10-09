

mod user;
use user::User;
fn show(user:&User){
    println!("{:?}",user)
}

fn main() {
    let users = User::new{
        id : 1,
        name : String::from("Bob"),
        age : 18,
    };
    show(&users);
}
