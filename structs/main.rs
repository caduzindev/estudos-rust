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