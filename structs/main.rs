#[derive(Debug)]
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

struct Color(i32,i32,i32);

fn main() {
  let username = String::from("carlos");
  let email = String::from("carlos@email.com");

  let user1 = build_user(username,email);
}

fn build_user(username: String, email: String) -> User {
  User {
    username,
    email,
    active: true,
    sign_in_count: 1,
  }
}

struct Point3<T> {
    x: T,
    y: T,
}

// Structs
impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
  let p = Point3 { x: 5, y: 10 };

  println!("p.x = {}", p.x());
}