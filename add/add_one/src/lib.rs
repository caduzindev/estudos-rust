use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn generate_number() -> i32 {
    rand::random::<i32>()
}
