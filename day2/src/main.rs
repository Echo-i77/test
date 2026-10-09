#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

impl User{
    fn birthday(&mut self){
        self.age+= 1;
    }
    fn show(&self){
        println!("{},{},{}",
        self.id,
        self.name,
        self.age );
    }
    fn is_adult(&self){
        self.age >= 18
    }
}
fn find_user(users:&[User],target_id:i32)->Option<&User>{
    for user in users{
        if user.id == target_id{
            return Some(user);
        }
    }
    None
}
fn main() {
    let mut users = vec![
        User {
            id: 1,
            name: String::from("Alan"),
            age: 18,
        },
        User {
            id: 2,
            name: String::from("Bob"),
            age: 17,
        },
        User {
            id: 3,
            name: String::from("Cindy"),
            age: 20,
        },
    ];
    match find_user(&users,2){
        Some(value) => println!(
            "user:{},{}",
            value.name,
            value.is_adult()
        ),
        None => println!("user not found"),
    }


}