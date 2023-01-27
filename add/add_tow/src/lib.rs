use add_one;

pub fn add_hola() -> String {
    let random = add_one::generate_number();

    let mut converter = random.to_string();
    converter.push_str("_hola");

    converter
}
