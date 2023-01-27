use add_one;
use add_tow;
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
    println!("Generate random number{}!", add_one::generate_number());
    println!("Generate random number {}", add_tow::add_hola());
}
