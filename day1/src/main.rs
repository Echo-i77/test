#[derive(Debug)]
struct User {
    id: i32,
    name : String,
    age : i32,
}
fn find_user(users:&Vec<User>, target_id:i32) -> Option<&User>{
    for user in &users{
        if user.id == target_id{
            return Some(user);
        }
    }
    None

}
impl User{
    fn is_adult(&self) -> bool{
        self.age>=18
    }
    fn show(&self){
        println!(
            "User {} ,Name {},Age {}",
            self.id,
            self.name,
            self.age
        );
    }
}
fn main() {
    let users = vec![
        User{
            id:1,
            name: String::from("Alan"),
            age: 18,
        },
        User{
            id:2,
            name: String::from("Bob"),
            age: 19,
        },
        User{
            id:3,
            name:String::from("Cindy"),
            age:20,
        },
    ];

    for user in users {
        if user.name == "Bob"{
            println!("Bob is adult? {}",user.is_adult());
            user.show();
        }
    }
}