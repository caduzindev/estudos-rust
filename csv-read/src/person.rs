use csv::StringRecord;

#[derive(Debug)]
pub struct Person {
    name: String,
    idade: u64,
}

impl Person {
    pub fn build_csv(args: &StringRecord) -> Person {
        let name = args[0].to_string();
        let idade = args[1].parse().unwrap();

        Person { name, idade }
    }
    pub fn is_adult(self) -> bool {
        if self.idade >= 18 {
            true
        } else {
            false
        }
    }
}
