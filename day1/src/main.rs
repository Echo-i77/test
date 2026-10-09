#[derive(Debug)]
struct User {
    id: i32,
    name : String,
    age : i32,
}
fn birthday(user:&mut User){
    let mut a: i32 = user.age;
    a+=1
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
    let mut users = vec![
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

    birthday(&mut user[0])
}