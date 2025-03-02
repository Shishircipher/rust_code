#[derive(Debug)] //// Enable Debug trait for println!
struct User {
        active : bool,
        username : String,
        email: &'static str,
        sign_in_count : u64,
    }
fn main(){

    let user1 = User {
        active : true,
        username : String::from("someusername123"),
        email : "someuser123@example.com",
        sign_in_count : 3,
    };

   // println!("{user1}")
      println!("{:?}", user1) //Debug println!
    
}
